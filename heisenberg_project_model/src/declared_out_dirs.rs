use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;

use cargo_metadata::Target;
use cargo_metadata::TargetKind;
use cargo_metadata::semver::Version;
use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;

use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::configuration::Configuration;
use crate::declared_build_script_output::DeclaredBuildScriptOutput;
use crate::declared_out_dir::DeclaredOutDir;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;

pub struct DeclaredOutDirs<'declaration> {
    pub configurations: &'declaration BTreeSet<Configuration>,
    pub manifest_path: &'declaration AbsPath,
    pub resolved: &'declaration ResolvedPackages,
}

impl DeclaredOutDirs<'_> {
    pub fn validated(
        &self,
        out_dirs: BTreeMap<String, DeclaredOutDir>,
    ) -> Result<BTreeMap<Configuration, BTreeMap<String, BuildScriptOutput>>, ProjectModelError>
    {
        let mut validated: BTreeMap<Configuration, BTreeMap<String, BuildScriptOutput>> = self
            .configurations
            .iter()
            .map(|configuration| (configuration.clone(), BTreeMap::new()))
            .collect();

        for (
            package,
            DeclaredOutDir {
                mut configurations,
                version,
            },
        ) in out_dirs
        {
            let PackageDocument { metadata, .. } = self.pinned_package(&package, &version)?;

            if !metadata
                .targets
                .iter()
                .any(|Target { kind, .. }| kind.contains(&TargetKind::CustomBuild))
            {
                return Err(ProjectModelError::OutDirPackageWithoutBuildScript {
                    manifest_path: self.manifest_path.to_path_buf(),
                    package,
                });
            }

            let mut declared = BTreeMap::new();

            for configuration in self.configurations {
                let Some(DeclaredBuildScriptOutput {
                    cfgs,
                    directory,
                    env,
                }) = configurations.remove(&configuration.name)
                else {
                    return Err(ProjectModelError::OutDirConfigurationUndeclared {
                        configuration: configuration.name.clone(),
                        manifest_path: self.manifest_path.to_path_buf(),
                        package,
                    });
                };
                let out_dir = self.out_dir(&configuration.name, &package, directory)?;

                declared.insert(
                    configuration.clone(),
                    BuildScriptOutput {
                        cfgs,
                        env,
                        out_dir: out_dir.into(),
                    },
                );
            }

            if let Some(configuration) = configurations.into_keys().next() {
                return Err(ProjectModelError::UnknownOutDirConfiguration {
                    configuration,
                    manifest_path: self.manifest_path.to_path_buf(),
                    package,
                });
            }

            for (configuration, output) in declared {
                validated
                    .entry(configuration)
                    .or_default()
                    .insert(metadata.id.repr.clone(), output);
            }
        }

        Ok(validated)
    }

    fn out_dir(
        &self,
        configuration: &str,
        package: &str,
        directory: String,
    ) -> Result<AbsPathBuf, ProjectModelError> {
        let out_dir = self.resolved.workspace.root.join(&directory).normalize();

        if !out_dir.starts_with(&self.resolved.workspace.root) {
            return Err(ProjectModelError::OutDirOutsideWorkspace {
                configuration: configuration.to_owned(),
                directory,
                manifest_path: self.manifest_path.to_path_buf(),
                package: package.to_owned(),
            });
        }

        if out_dir.starts_with(&self.resolved.workspace.target_directory) {
            return Err(ProjectModelError::OutDirInsideTargetDirectory {
                configuration: configuration.to_owned(),
                directory,
                manifest_path: self.manifest_path.to_path_buf(),
                package: package.to_owned(),
            });
        }

        fs::read_dir(&out_dir)
            .map_err(|source| ProjectModelError::UnreadableOutDir {
                path: out_dir.clone().into(),
                source,
            })
            .map(|_| out_dir)
    }

    fn pinned_package(
        &self,
        package: &str,
        version: &Version,
    ) -> Result<&PackageDocument, ProjectModelError> {
        let named: Vec<&PackageDocument> = self.resolved.named(package).collect();

        match named
            .iter()
            .copied()
            .filter(|PackageDocument { metadata, .. }| metadata.version == *version)
            .collect::<Vec<&PackageDocument>>()
            .as_slice()
        {
            [] if named.is_empty() => Err(ProjectModelError::UnknownOutDirPackage {
                manifest_path: self.manifest_path.to_path_buf(),
                package: package.to_owned(),
            }),
            [] => Err(ProjectModelError::OutDirPackageVersionUnresolved {
                declared: Box::new(version.clone()),
                manifest_path: self.manifest_path.to_path_buf(),
                package: package.to_owned(),
                resolved: named
                    .iter()
                    .map(|PackageDocument { metadata, .. }| metadata.version.to_string())
                    .collect(),
            }),
            [document] => Ok(*document),
            documents => Err(ProjectModelError::AmbiguousOutDirPackage {
                manifest_path: self.manifest_path.to_path_buf(),
                package: package.to_owned(),
                packages: documents
                    .iter()
                    .map(|PackageDocument { metadata, .. }| metadata.id.repr.clone())
                    .collect(),
                version: Box::new(version.clone()),
            }),
        }
    }
}
