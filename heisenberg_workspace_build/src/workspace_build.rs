use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::hash_map::Entry;
use std::io;
use std::mem;
use std::sync::Arc;

use cargo::GlobalContext;
use cargo::compiler::BuildConfig;
use cargo::compiler::BuildContext;
use cargo::compiler::BuildRunner;
use cargo::compiler::CompileMode;
use cargo::compiler::DefaultExecutor;
use cargo::compiler::Executor;
use cargo::compiler::MessageFormat;
use cargo::compiler::Unit;
use cargo::compiler::UnitInterner;
use cargo::compiler::UserIntent;
use cargo::compiler::unit_graph::UnitDep;
use cargo::compiler::unit_graph::UnitGraph;
use cargo::ops::CompileFilter;
use cargo::ops::CompileOptions;
use cargo::ops::Packages;
use cargo::ops::create_bcx;
use cargo::resolver::features::CliFeatures;
use cargo::workspace::Workspace;
use cargo_metadata::Artifact;
use cargo_metadata::BuildScript;
use cargo_metadata::Message;
use cargo_metadata::TargetKind;
use cargo_metadata::camino::Utf8PathBuf;

use crate::build_outputs::BuildOutputs;
use crate::build_script_output::BuildScriptOutput;
use crate::captured_output::CapturedOutput;
use crate::cargo_session::CargoSession;
use crate::proc_macro_library::proc_macro_library;
use crate::workspace_build_error::WorkspaceBuildError;

const fn build_script(message: &Message) -> Option<&BuildScript> {
    match message {
        Message::BuildScriptExecuted(executed) => Some(executed),
        _ => None,
    }
}

fn build_scripts(host: &[Message], target: &[Message]) -> BTreeMap<String, BuildScriptOutput> {
    let host_runs: BTreeSet<&Utf8PathBuf> = host
        .iter()
        .filter_map(build_script)
        .map(|BuildScript { out_dir, .. }| out_dir)
        .collect();
    let latest_runs: BTreeMap<&Utf8PathBuf, &BuildScript> = host
        .iter()
        .chain(target)
        .filter_map(build_script)
        .map(|run| (&run.out_dir, run))
        .collect();

    latest_runs
        .values()
        .filter(|BuildScript { out_dir, .. }| host_runs.contains(out_dir))
        .chain(
            latest_runs
                .values()
                .filter(|BuildScript { out_dir, .. }| !host_runs.contains(out_dir)),
        )
        .map(|run| (run.package_id.repr.clone(), BuildScriptOutput::from(*run)))
        .collect()
}

fn diagnostic(message: Message) -> Option<String> {
    match message {
        Message::CompilerMessage(compiled) => compiled.message.rendered,
        Message::TextLine(line) => Some(format!("{line}\n")),
        _ => None,
    }
}

fn is_built_for_the_host(unit: &Unit) -> bool {
    unit.kind.is_host() && (unit.mode.is_run_custom_build() || is_proc_macro_dylib(unit))
}

fn is_proc_macro_dylib(unit: &Unit) -> bool {
    unit.mode == CompileMode::Build && unit.target.proc_macro()
}

fn is_run_for_the_target(unit: &Unit) -> bool {
    !unit.kind.is_host() && unit.mode.is_run_custom_build()
}

fn proc_macro_artifact(message: &Message) -> Option<&Artifact> {
    match message {
        Message::CompilerArtifact(artifact)
            if artifact.target.kind.contains(&TargetKind::ProcMacro) =>
        {
            Some(artifact)
        }
        _ => None,
    }
}

fn proc_macro_dylibs(
    host: &[Message],
) -> Result<BTreeMap<Utf8PathBuf, Utf8PathBuf>, WorkspaceBuildError> {
    host.iter()
        .filter_map(proc_macro_artifact)
        .map(
            |Artifact {
                 filenames, target, ..
             }| {
                proc_macro_library(&target.name, filenames)
                    .map(|library| (target.src_path.clone(), library))
            },
        )
        .collect()
}

pub struct WorkspaceBuild<'build> {
    pub features: &'build CliFeatures,
    pub session: &'build CargoSession,
    pub substituted_packages: &'build BTreeSet<String>,
    pub target: &'build str,
    pub workspace: &'build Workspace<'build>,
}

impl WorkspaceBuild<'_> {
    /// # Errors
    ///
    /// Returns [`WorkspaceBuildError`] when cargo cannot plan, build or run the build
    /// scripts and procedural macros of the workspace for the target, or when a build
    /// script that runs needs one that the workspace declares instead.
    pub fn outputs(&self) -> Result<BuildOutputs, WorkspaceBuildError> {
        let CargoSession { captured, context } = self.session;

        self.workspace_outputs(context, self.workspace, captured)
    }

    fn closure(&self, graph: &UnitGraph, roots: &[Unit]) -> Result<UnitGraph, WorkspaceBuildError> {
        let mut closure = UnitGraph::default();
        let mut pending = roots.to_vec();

        while let Some(unit) = pending.pop() {
            if let Entry::Vacant(vacancy) = closure.entry(unit) {
                let dependencies = graph[vacancy.key()].clone();

                if let Some(substituted) = dependencies
                    .iter()
                    .find(|UnitDep { unit, .. }| self.is_substituted(unit))
                {
                    return Err(WorkspaceBuildError::SubstitutedBuildScriptRequired {
                        dependent: vacancy.key().pkg.name().to_string(),
                        substituted: substituted.unit.pkg.name().to_string(),
                    });
                }

                pending.extend(dependencies.iter().map(|UnitDep { unit, .. }| unit.clone()));
                vacancy.insert(dependencies);
            }
        }

        Ok(closure)
    }

    fn is_substituted(&self, unit: &Unit) -> bool {
        unit.mode.is_run_custom_build()
            && self
                .substituted_packages
                .contains(&unit.pkg.package_id().to_spec().to_string())
    }

    fn messages(
        &self,
        plan: &mut BuildContext<'_, '_>,
        graph: &UnitGraph,
        selects: fn(&Unit) -> bool,
        captured: &CapturedOutput,
    ) -> Result<Vec<Message>, WorkspaceBuildError> {
        let roots: Vec<Unit> = graph
            .keys()
            .filter(|unit| selects(unit) && !self.is_substituted(unit))
            .cloned()
            .collect();
        let executor: Arc<dyn Executor> = Arc::new(DefaultExecutor);

        self.closure(graph, &roots).and_then(|closure| {
            plan.unit_graph = closure;
            plan.roots = roots;

            BuildRunner::new(plan)
                .and_then(|runner| runner.compile(&executor))
                .map_err(|source| WorkspaceBuildError::Build {
                    output: Message::parse_stream(captured.take().as_slice())
                        .map_while(Result::ok)
                        .filter_map(diagnostic)
                        .collect(),
                    source: source.into(),
                })
                .and_then(|_| {
                    plan.gctx
                        .shell()
                        .out()
                        .flush()
                        .map_err(WorkspaceBuildError::BuildMessages)
                })
                .and_then(|()| {
                    Message::parse_stream(captured.take().as_slice())
                        .collect::<Result<Vec<Message>, io::Error>>()
                        .map_err(WorkspaceBuildError::BuildMessages)
                })
        })
    }

    fn planned_outputs(
        &self,
        plan: &mut BuildContext<'_, '_>,
        captured: &CapturedOutput,
    ) -> Result<BuildOutputs, WorkspaceBuildError> {
        let graph = mem::take(&mut plan.unit_graph);

        self.messages(plan, &graph, is_built_for_the_host, captured)
            .and_then(|host| {
                self.messages(plan, &graph, is_run_for_the_target, captured)
                    .and_then(|target| {
                        proc_macro_dylibs(&host).map(|proc_macro_dylibs| BuildOutputs {
                            build_scripts: build_scripts(&host, &target),
                            proc_macro_dylibs,
                        })
                    })
            })
    }

    fn workspace_outputs(
        &self,
        context: &GlobalContext,
        workspace: &Workspace<'_>,
        captured: &CapturedOutput,
    ) -> Result<BuildOutputs, WorkspaceBuildError> {
        BuildConfig::new(
            context,
            None,
            false,
            &[self.target.to_owned()],
            UserIntent::Check { test: false },
        )
        .map_err(Into::into)
        .map_err(WorkspaceBuildError::BuildConfiguration)
        .and_then(|mut build_config| {
            build_config.message_format = MessageFormat::Json {
                render_diagnostics: false,
                short: false,
                ansi: false,
            };

            let options = CompileOptions {
                build_config,
                cli_features: self.features.clone(),
                spec: Packages::All(Vec::new()),
                filter: CompileFilter::new_all_targets(),
                target_rustdoc_args: None,
                target_rustc_args: None,
                target_rustc_crate_types: None,
                rustdoc_document_private_items: false,
                honor_rust_version: None,
            };
            let interner = UnitInterner::new();

            create_bcx(workspace, &options, &interner, None)
                .map_err(Into::into)
                .map_err(WorkspaceBuildError::BuildPlan)
                .and_then(|mut plan| {
                    plan.target_data
                        .requested_kinds()
                        .iter()
                        .map(|kind| plan.target_data.info(*kind).sysroot_target_libdir.clone())
                        .find(|directory| !directory.is_dir())
                        .map_or_else(
                            || self.planned_outputs(&mut plan, captured),
                            |directory| {
                                Err(WorkspaceBuildError::TargetLibraryMissing {
                                    directory,
                                    target: self.target.to_owned(),
                                })
                            },
                        )
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::io;

    use cargo_metadata::Message;
    use cargo_metadata::camino::Utf8PathBuf;

    use super::build_scripts;
    use crate::build_script_output::BuildScriptOutput;

    const HOST_RUN: &str = r#"{"reason":"build-script-executed","package_id":"path+file:///workspace/generated#0.1.0","linked_libs":[],"linked_paths":[],"cfgs":["host"],"env":[["GENERATED","host"]],"out_dir":"/workspace/target/debug/build/generated-host/out"}"#;
    const HOST_RUN_REPEATED_WITH_A_NEW_OUTPUT: &str = r#"{"reason":"build-script-executed","package_id":"path+file:///workspace/generated#0.1.0","linked_libs":[],"linked_paths":[],"cfgs":["host"],"env":[["GENERATED","repeated"]],"out_dir":"/workspace/target/debug/build/generated-host/out"}"#;
    const PACKAGE: &str = "path+file:///workspace/generated#0.1.0";
    const TARGET_RUN: &str = r#"{"reason":"build-script-executed","package_id":"path+file:///workspace/generated#0.1.0","linked_libs":[],"linked_paths":[],"cfgs":["target"],"env":[["GENERATED","target"]],"out_dir":"/workspace/target/x86_64-unknown-linux-gnu/debug/build/generated-target/out"}"#;

    fn attributed(host: &[&str], target: &[&str]) -> Option<BTreeMap<String, BuildScriptOutput>> {
        messages(host)
            .and_then(|host| messages(target).map(|target| build_scripts(&host, &target)))
            .ok()
    }

    fn messages(lines: &[&str]) -> Result<Vec<Message>, io::Error> {
        Message::parse_stream(lines.join("\n").as_bytes()).collect()
    }

    fn output(cfg: &str, generated: &str, out_dir: &str) -> BTreeMap<String, BuildScriptOutput> {
        BTreeMap::from([(
            PACKAGE.to_owned(),
            BuildScriptOutput {
                cfgs: vec![cfg.to_owned()],
                env: BTreeMap::from([("GENERATED".to_owned(), generated.to_owned())]),
                out_dir: Utf8PathBuf::from(out_dir),
            },
        )])
    }

    #[test]
    fn target_run_of_a_build_script_wins_over_its_host_run_reported_after_it() {
        assert_eq!(
            attributed(&[HOST_RUN], &[TARGET_RUN, HOST_RUN]),
            Some(output(
                "target",
                "target",
                "/workspace/target/x86_64-unknown-linux-gnu/debug/build/generated-target/out"
            ))
        );
    }

    #[test]
    fn target_run_of_a_build_script_wins_over_its_host_run_repeated_with_a_new_output() {
        assert_eq!(
            attributed(
                &[HOST_RUN],
                &[TARGET_RUN, HOST_RUN_REPEATED_WITH_A_NEW_OUTPUT]
            ),
            Some(output(
                "target",
                "target",
                "/workspace/target/x86_64-unknown-linux-gnu/debug/build/generated-target/out"
            ))
        );
    }

    #[test]
    fn host_run_repeated_with_a_new_output_replaces_its_earlier_output() {
        assert_eq!(
            attributed(&[HOST_RUN], &[HOST_RUN_REPEATED_WITH_A_NEW_OUTPUT]),
            Some(output(
                "host",
                "repeated",
                "/workspace/target/debug/build/generated-host/out"
            ))
        );
    }
}
