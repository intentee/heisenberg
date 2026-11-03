use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::hash_map::Entry;
use std::io;
use std::mem;
use std::path::Path;
use std::sync::Arc;

use cargo::GlobalContext;
use cargo::compiler::BuildConfig;
use cargo::compiler::BuildContext;
use cargo::compiler::BuildRunner;
use cargo::compiler::CompileMode;
use cargo::compiler::CrateType;
use cargo::compiler::DefaultExecutor;
use cargo::compiler::Executor;
use cargo::compiler::FileFlavor;
use cargo::compiler::MessageFormat;
use cargo::compiler::Unit;
use cargo::compiler::UnitInterner;
use cargo::compiler::UserIntent;
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
use cargo_util_terminal::Shell;

use crate::build_outputs::BuildOutputs;
use crate::build_script_output::BuildScriptOutput;
use crate::captured_output::CapturedOutput;
use crate::configured_context::ConfiguredContext;
use crate::workspace_build_error::WorkspaceBuildError;

const fn build_script(message: &Message) -> Option<&BuildScript> {
    match message {
        Message::BuildScriptExecuted(executed) => Some(executed),
        _ => None,
    }
}

fn build_script_outputs(messages: &[Message]) -> BTreeMap<String, BuildScriptOutput> {
    messages
        .iter()
        .filter_map(build_script)
        .map(|executed| {
            (
                executed.package_id.repr.clone(),
                BuildScriptOutput::from(executed),
            )
        })
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

pub struct WorkspaceBuild<'build> {
    pub context: &'build ConfiguredContext<'build>,
    pub manifest_path: &'build Path,
    pub substituted_packages: &'build BTreeSet<String>,
    pub target: &'build str,
}

impl WorkspaceBuild<'_> {
    /// # Errors
    ///
    /// Returns [`WorkspaceBuildError`] when cargo cannot plan, build or run the build
    /// scripts and procedural macros of the workspace for the target, or when a build
    /// script that runs needs one that the workspace declares instead.
    pub fn outputs(&self) -> Result<BuildOutputs, WorkspaceBuildError> {
        let captured = CapturedOutput::default();

        self.context
            .with_shell(Shell::from_write(Box::new(captured.clone())))
            .map_err(Into::into)
            .map_err(WorkspaceBuildError::CargoConfiguration)
            .and_then(|context| {
                Workspace::new(self.manifest_path, &context)
                    .map_err(Into::into)
                    .map_err(WorkspaceBuildError::CargoWorkspace)
                    .and_then(|workspace| self.workspace_outputs(&context, &workspace, &captured))
            })
    }

    fn closure(&self, graph: &UnitGraph, roots: &[Unit]) -> Result<UnitGraph, WorkspaceBuildError> {
        let mut closure = UnitGraph::default();
        let mut pending = roots.to_vec();

        while let Some(unit) = pending.pop() {
            if let Entry::Vacant(vacancy) = closure.entry(unit) {
                let dependencies = graph[vacancy.key()].clone();

                if let Some(substituted) = dependencies
                    .iter()
                    .find(|dependency| self.is_substituted(&dependency.unit))
                {
                    return Err(WorkspaceBuildError::SubstitutedBuildScriptRequired {
                        dependent: vacancy.key().pkg.name().to_string(),
                        substituted: substituted.unit.pkg.name().to_string(),
                    });
                }

                pending.extend(
                    dependencies
                        .iter()
                        .map(|dependency| dependency.unit.clone()),
                );
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
        context: &GlobalContext,
        plan: &mut BuildContext<'_, '_>,
        captured: &CapturedOutput,
    ) -> Result<BuildOutputs, WorkspaceBuildError> {
        let graph = mem::take(&mut plan.unit_graph);

        self.messages(plan, &graph, is_built_for_the_host, captured)
            .and_then(|host| {
                self.messages(plan, &graph, is_run_for_the_target, captured)
                    .and_then(|target| {
                        Self::proc_macro_dylibs(context, plan, &graph, &host).map(
                            |proc_macro_dylibs| {
                                let host_scripts = build_script_outputs(&host);

                                BuildOutputs {
                                    build_scripts: host_scripts
                                        .clone()
                                        .into_iter()
                                        .chain(build_script_outputs(&target).into_iter().filter(
                                            |(package, output)| {
                                                host_scripts.get(package) != Some(output)
                                            },
                                        ))
                                        .collect(),
                                    proc_macro_dylibs,
                                }
                            },
                        )
                    })
            })
    }

    fn proc_macro_dylibs(
        context: &GlobalContext,
        plan: &BuildContext<'_, '_>,
        graph: &UnitGraph,
        host: &[Message],
    ) -> Result<BTreeMap<Utf8PathBuf, Utf8PathBuf>, WorkspaceBuildError> {
        let artifacts: BTreeMap<String, &Artifact> = host
            .iter()
            .filter_map(proc_macro_artifact)
            .map(|artifact| (artifact.package_id.repr.clone(), artifact))
            .collect();

        graph
            .keys()
            .filter(|unit| is_proc_macro_dylib(unit))
            .map(|unit| {
                plan.target_data
                    .info(unit.kind)
                    .rustc_outputs(
                        unit.mode,
                        unit.target.kind(),
                        plan.target_data.short_name(&unit.kind),
                        context,
                    )
                    .map_err(Into::into)
                    .map_err(WorkspaceBuildError::ProcMacroFileTypes)
                    .map(|(file_types, _)| {
                        let artifact = artifacts[&unit.pkg.package_id().to_spec().to_string()];

                        file_types
                            .iter()
                            .filter(|file_type| {
                                file_type.flavor == FileFlavor::Linkable
                                    && file_type.crate_type == Some(CrateType::ProcMacro)
                            })
                            .flat_map(|file_type| {
                                let (prefix, suffix) = file_type.output_prefix_suffix(&unit.target);

                                artifact
                                    .filenames
                                    .iter()
                                    .filter(move |file| {
                                        file.file_name().is_some_and(|name| {
                                            name.starts_with(&prefix) && name.ends_with(&suffix)
                                        })
                                    })
                                    .map(|dylib| (artifact.target.src_path.clone(), dylib.clone()))
                                    .collect::<BTreeMap<Utf8PathBuf, Utf8PathBuf>>()
                            })
                            .collect::<BTreeMap<Utf8PathBuf, Utf8PathBuf>>()
                    })
            })
            .collect::<Result<Vec<BTreeMap<Utf8PathBuf, Utf8PathBuf>>, WorkspaceBuildError>>()
            .map(|dylibs| dylibs.into_iter().flatten().collect())
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
                cli_features: CliFeatures::new_all(true),
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
                            || self.planned_outputs(context, &mut plan, captured),
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
