use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;

use cargo_metadata::TargetKind;
use ra_ap_paths::AbsPath;

use heisenberg_error_context::error_context::error_context;

use crate::declared_out_dir::DeclaredOutDir;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::substituted_build_script::SubstitutedBuildScript;
use crate::supported_target::SupportedTarget;

pub struct DeclaredOutDirs<'declaration> {
    pub manifest_path: &'declaration AbsPath,
    pub resolved: &'declaration ResolvedPackages,
    pub targets: &'declaration BTreeSet<SupportedTarget>,
}

impl DeclaredOutDirs<'_> {
    pub fn validated(
        &self,
        out_dirs: BTreeMap<String, DeclaredOutDir>,
    ) -> Result<BTreeMap<String, SubstitutedBuildScript>, ProjectModelError> {
        let mut validated = BTreeMap::new();

        for (
            package,
            DeclaredOutDir {
                directory,
                mut targets,
            },
        ) in out_dirs
        {
            let PackageDocument { metadata, .. } = match self
                .resolved
                .packages
                .iter()
                .filter(|document| document.metadata.name.as_str() == package)
                .collect::<Vec<&PackageDocument>>()
                .as_slice()
            {
                [] => {
                    return Err(ProjectModelError::UnknownOutDirPackage {
                        manifest_path: self.manifest_path.to_path_buf(),
                        package,
                    });
                }
                [document] => *document,
                documents => {
                    return Err(ProjectModelError::AmbiguousOutDirPackage {
                        manifest_path: self.manifest_path.to_path_buf(),
                        versions: documents
                            .iter()
                            .map(|document| document.metadata.version.to_string())
                            .collect(),
                        package,
                    });
                }
            };

            if !metadata
                .targets
                .iter()
                .any(|target| target.kind.contains(&TargetKind::CustomBuild))
            {
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

            let mut declared = BTreeMap::new();

            for target in self.targets {
                let target_directory = base.join(target.triple());

                fs::read_dir(&target_directory).map_err(error_context(
                    target_directory.into(),
                    ProjectModelError::UnreadableOutDir,
                ))?;

                let Some(output) = targets.remove(target.triple()) else {
                    return Err(ProjectModelError::OutDirTargetUndeclared {
                        manifest_path: self.manifest_path.to_path_buf(),
                        package,
                        target: target.triple().to_owned(),
                    });
                };

                declared.insert(*target, output);
            }

            if let Some(target) = targets.into_keys().next() {
                return Err(ProjectModelError::UnknownOutDirTarget {
                    manifest_path: self.manifest_path.to_path_buf(),
                    package,
                    target,
                });
            }

            validated.insert(
                metadata.id.repr.clone(),
                SubstitutedBuildScript {
                    directory: base,
                    targets: declared,
                },
            );
        }

        Ok(validated)
    }
}
