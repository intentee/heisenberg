use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;

use heisenberg_error_context::error_context::error_context;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

use crate::member_target::MemberTarget;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::supported_target::SupportedTarget;

pub struct DeclaredOutDirs<'declaration> {
    pub manifest_path: &'declaration AbsPath,
    pub resolved: &'declaration ResolvedPackages,
    pub targets: &'declaration BTreeSet<SupportedTarget>,
}

impl DeclaredOutDirs<'_> {
    pub fn validated(
        &self,
        out_dirs: BTreeMap<String, String>,
    ) -> Result<BTreeMap<String, AbsPathBuf>, ProjectModelError> {
        let mut validated = BTreeMap::new();

        for (package, directory) in out_dirs {
            let kinds: Vec<CrateTargetKind> = self
                .resolved
                .member_targets
                .iter()
                .filter(|MemberTarget { crate_target, .. }| crate_target.package_name == package)
                .map(|MemberTarget { crate_target, .. }| crate_target.kind)
                .collect();

            if kinds.is_empty() {
                return Err(ProjectModelError::UnknownOutDirPackage {
                    manifest_path: self.manifest_path.to_path_buf(),
                    package,
                });
            }

            if !kinds.contains(&CrateTargetKind::BuildScript) {
                return Err(ProjectModelError::OutDirPackageWithoutBuildScript {
                    manifest_path: self.manifest_path.to_path_buf(),
                    package,
                });
            }

            let base = self.resolved.workspace.root.join(&directory).normalize();

            if !base.starts_with(&self.resolved.workspace.root) {
                return Err(ProjectModelError::OutDirOutsideWorkspace {
                    directory,
                    manifest_path: self.manifest_path.to_path_buf(),
                    package,
                });
            }

            for target in self.targets {
                let target_directory = base.join(target.triple());

                fs::read_dir(&target_directory).map_err(error_context(
                    target_directory.into(),
                    ProjectModelError::UnreadableOutDir,
                ))?;
            }

            validated.insert(package, base);
        }

        Ok(validated)
    }
}
