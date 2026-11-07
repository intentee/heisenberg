use std::collections::BTreeMap;
use std::collections::BTreeSet;

use cargo_metadata::Target;
use cargo_metadata::TargetKind;
use ra_ap_paths::AbsPath;
use ra_ap_toolchain::Tool;

use heisenberg_error_context::error_context::error_context;
use heisenberg_workspace_build::build_outputs::BuildOutputs;
use heisenberg_workspace_build::cargo_session::CargoSession;
use heisenberg_workspace_build::configured_context::ConfiguredContext;
use heisenberg_workspace_build::workspace_build::WorkspaceBuild;

use crate::configuration::Configuration;
use crate::constants::RUSTC_VARIABLE;
use crate::constants::SYSROOT_BINARY_DIRECTORY;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::substituted_build_script::SubstitutedBuildScript;

fn with_substituted(
    configuration: &Configuration,
    mut outputs: BuildOutputs,
    substituted: &BTreeMap<String, SubstitutedBuildScript>,
) -> BuildOutputs {
    outputs.build_scripts.extend(
        substituted
            .iter()
            .map(|(package, script)| (package.clone(), script.output(configuration))),
    );

    outputs
}

pub struct ConfigurationBuilds<'builds> {
    pub configurations: &'builds BTreeSet<Configuration>,
    pub context: &'builds ConfiguredContext<'builds>,
    pub manifest_path: &'builds AbsPath,
    pub resolved: &'builds ResolvedPackages,
    pub substituted: &'builds BTreeMap<String, SubstitutedBuildScript>,
    pub sysroot: &'builds AbsPath,
}

impl ConfigurationBuilds<'_> {
    pub fn outputs(&self) -> Result<BTreeMap<Configuration, BuildOutputs>, ProjectModelError> {
        let substituted_packages: BTreeSet<String> = self.substituted.keys().cloned().collect();

        if self.builds_anything(&substituted_packages) {
            let directory = self.sysroot.join(SYSROOT_BINARY_DIRECTORY);
            let compiler = Tool::Rustc.path_in(directory.as_ref()).ok_or_else(|| {
                ProjectModelError::SysrootWithoutCompiler {
                    directory: directory.clone(),
                }
            })?;
            let mut environment = self.context.environment.clone();

            environment.insert(RUSTC_VARIABLE.to_owned(), compiler.into_string());

            let context = ConfiguredContext {
                cargo_home: self.context.cargo_home,
                environment: &environment,
                nightly_features_allowed: self.context.nightly_features_allowed,
                workspace_directory: self.context.workspace_directory,
            };

            CargoSession::new(&context)
                .map_err(ProjectModelError::WorkspaceBuildSession)
                .and_then(|session| {
                    session
                        .workspace(self.manifest_path.as_ref())
                        .map_err(ProjectModelError::WorkspaceBuildSession)
                        .and_then(|workspace| {
                            self.configurations
                                .iter()
                                .map(|configuration| {
                                    WorkspaceBuild {
                                        features: &configuration.features.cli_features(),
                                        session: &session,
                                        substituted_packages: &substituted_packages,
                                        target: configuration.target.triple(),
                                        workspace: &workspace,
                                    }
                                    .outputs()
                                    .map_err(Box::new)
                                    .map_err(error_context(
                                        configuration.clone(),
                                        ProjectModelError::WorkspaceBuild,
                                    ))
                                    .map(|outputs| {
                                        (
                                            configuration.clone(),
                                            with_substituted(
                                                configuration,
                                                outputs,
                                                self.substituted,
                                            ),
                                        )
                                    })
                                })
                                .collect()
                        })
                })
        } else {
            Ok(self
                .configurations
                .iter()
                .map(|configuration| {
                    (
                        configuration.clone(),
                        with_substituted(configuration, BuildOutputs::default(), self.substituted),
                    )
                })
                .collect())
        }
    }

    fn builds_anything(&self, substituted_packages: &BTreeSet<String>) -> bool {
        self.resolved
            .packages
            .iter()
            .any(|PackageDocument { metadata, .. }| {
                metadata.targets.iter().any(|Target { kind, .. }| {
                    kind.contains(&TargetKind::ProcMacro)
                        || (kind.contains(&TargetKind::CustomBuild)
                            && !substituted_packages.contains(&metadata.id.repr))
                })
            })
    }
}
