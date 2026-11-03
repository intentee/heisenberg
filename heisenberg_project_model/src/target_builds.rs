use std::collections::BTreeMap;
use std::collections::BTreeSet;

use cargo_metadata::Target;
use cargo_metadata::TargetKind;
use ra_ap_paths::AbsPath;

use heisenberg_error_context::error_context::error_context;
use heisenberg_workspace_build::build_outputs::BuildOutputs;
use heisenberg_workspace_build::configured_context::ConfiguredContext;
use heisenberg_workspace_build::workspace_build::WorkspaceBuild;

use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::substituted_build_script::SubstitutedBuildScript;
use crate::supported_target::SupportedTarget;

pub struct TargetBuilds<'builds> {
    pub context: &'builds ConfiguredContext<'builds>,
    pub manifest_path: &'builds AbsPath,
    pub resolved: &'builds ResolvedPackages,
    pub substituted: &'builds BTreeMap<String, SubstitutedBuildScript>,
    pub targets: &'builds BTreeSet<SupportedTarget>,
}

impl TargetBuilds<'_> {
    pub fn outputs(&self) -> Result<BTreeMap<SupportedTarget, BuildOutputs>, ProjectModelError> {
        let substituted_packages: BTreeSet<String> = self.substituted.keys().cloned().collect();
        let builds_anything =
            self.resolved
                .packages
                .iter()
                .any(|PackageDocument { metadata, .. }| {
                    metadata.targets.iter().any(|Target { kind, .. }| {
                        kind.contains(&TargetKind::ProcMacro)
                            || (kind.contains(&TargetKind::CustomBuild)
                                && !substituted_packages.contains(&metadata.id.repr))
                    })
                });

        self.targets
            .iter()
            .map(|target| {
                if builds_anything {
                    WorkspaceBuild {
                        context: self.context,
                        manifest_path: self.manifest_path.as_ref(),
                        substituted_packages: &substituted_packages,
                        target: target.triple(),
                    }
                    .outputs()
                    .map_err(error_context(*target, ProjectModelError::WorkspaceBuild))
                } else {
                    Ok(BuildOutputs::default())
                }
                .map(|mut outputs| {
                    outputs.build_scripts.extend(
                        self.substituted
                            .iter()
                            .map(|(package, script)| (package.clone(), script.output(*target))),
                    );

                    (*target, outputs)
                })
            })
            .collect()
    }
}
