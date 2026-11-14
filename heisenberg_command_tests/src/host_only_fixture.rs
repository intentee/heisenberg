use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use toml::Table;
use toml::Value;
use toml::to_string;

use crate::cleared_path::cleared_path;
use crate::constants::CARGO_BUILD_TABLE;
use crate::constants::CARGO_CONFIGURATION_DIRECTORY;
use crate::constants::CARGO_CONFIGURATION_FILE;
use crate::constants::CARGO_TARGET_DIRECTORY_KEY;
use crate::constants::HOST_ONLY_BUILDS_DIRECTORY;
use crate::constants::HOST_TRIPLE;
use crate::fixture_configuration::FixtureConfiguration;
use crate::fixture_out_dir::FixtureOutDir;
use crate::fixture_workspace::FixtureWorkspace;
use crate::heisenberg_check_error::HeisenbergCheckError;
use crate::host_configuration::HostConfiguration;

fn copied_entry(entry: &DirEntry, destination: &Path) -> Result<(), HeisenbergCheckError> {
    let copy = destination.join(entry.file_name());

    entry
        .file_type()
        .map_err(|source| HeisenbergCheckError::FixtureCopy {
            path: entry.path(),
            source,
        })
        .and_then(|file_type| {
            if file_type.is_dir() {
                copied_directory(&entry.path(), &copy)
            } else {
                fs::copy(entry.path(), &copy)
                    .map(drop)
                    .map_err(|source| HeisenbergCheckError::FixtureCopy {
                        path: entry.path(),
                        source,
                    })
                    .and_then(|()| preserved_timestamp(&entry.path(), &copy))
            }
        })
}

fn copied_directory(source: &Path, destination: &Path) -> Result<(), HeisenbergCheckError> {
    fs::create_dir(destination)
        .map_err(|source| HeisenbergCheckError::FixtureCopy {
            path: destination.to_path_buf(),
            source,
        })
        .and_then(|()| {
            fs::read_dir(source).map_err(|error| HeisenbergCheckError::FixtureCopy {
                path: source.to_path_buf(),
                source: error,
            })
        })
        .and_then(|entries| {
            entries.into_iter().try_for_each(|entry| {
                entry
                    .map_err(|error| HeisenbergCheckError::FixtureCopy {
                        path: source.to_path_buf(),
                        source: error,
                    })
                    .and_then(|entry| copied_entry(&entry, destination))
            })
        })
}

fn preserved_timestamp(source: &Path, copy: &Path) -> Result<(), HeisenbergCheckError> {
    fs::metadata(source)
        .and_then(|metadata| metadata.modified())
        .and_then(|modified| {
            File::options()
                .write(true)
                .open(copy)
                .and_then(|file| file.set_modified(modified))
        })
        .map_err(|error| HeisenbergCheckError::FixtureTimestamp {
            path: source.to_path_buf(),
            source: error,
        })
}

fn written_cargo_configuration(
    copy: &Path,
    build_directory: PathBuf,
    timestamp_source: &Path,
) -> Result<(), HeisenbergCheckError> {
    let directory = copy.join(CARGO_CONFIGURATION_DIRECTORY);
    let file = directory.join(CARGO_CONFIGURATION_FILE);

    build_directory
        .into_os_string()
        .into_string()
        .map_err(PathBuf::from)
        .map_err(HeisenbergCheckError::FixtureBuildDirectoryNotUtf8)
        .and_then(|build_directory| {
            let mut build = Table::new();
            let mut configuration = Table::new();

            build.insert(
                CARGO_TARGET_DIRECTORY_KEY.to_owned(),
                Value::String(build_directory),
            );
            configuration.insert(CARGO_BUILD_TABLE.to_owned(), Value::Table(build));

            to_string(&configuration)
                .map_err(HeisenbergCheckError::FixtureCargoConfigurationSerialization)
        })
        .and_then(|text| {
            fs::create_dir(&directory)
                .and_then(|()| fs::write(&file, text))
                .map_err(
                    |source| HeisenbergCheckError::FixtureCargoConfigurationWrite {
                        path: file.clone(),
                        source,
                    },
                )
        })
        .and_then(|()| preserved_timestamp(timestamp_source, &file))
}

fn host_configuration(
    table: Table,
    index: usize,
    manifest_path: &Path,
) -> Result<Option<HostConfiguration>, HeisenbergCheckError> {
    let context = FixtureConfiguration {
        index,
        manifest_path: manifest_path.to_path_buf(),
    };

    table
        .get("target")
        .cloned()
        .ok_or(HeisenbergCheckError::FixtureConfigurationTargetMissing(
            context.clone(),
        ))
        .and_then(|target| {
            target
                .try_into::<String>()
                .map_err(Box::new)
                .map_err(|source| HeisenbergCheckError::FixtureConfigurationTarget {
                    configuration: context.clone(),
                    source,
                })
        })
        .and_then(move |target| {
            table
                .get("name")
                .cloned()
                .ok_or(HeisenbergCheckError::FixtureConfigurationNameMissing(
                    context.clone(),
                ))
                .and_then(|name| {
                    name.try_into::<String>()
                        .map_err(Box::new)
                        .map_err(|source| HeisenbergCheckError::FixtureConfigurationName {
                            configuration: context,
                            source,
                        })
                })
                .map(move |name| {
                    (target == HOST_TRIPLE).then_some(HostConfiguration { name, table })
                })
        })
}

fn host_out_dir(
    package: &str,
    out_dir: &mut Value,
    host_names: &[String],
    manifest_path: &Path,
) -> Result<(), HeisenbergCheckError> {
    out_dir["configurations"]
        .clone()
        .try_into::<Table>()
        .map_err(Box::new)
        .map_err(|source| HeisenbergCheckError::FixtureOutDirConfigurations {
            out_dir: FixtureOutDir {
                manifest_path: manifest_path.to_path_buf(),
                package: package.to_owned(),
            },
            source,
        })
        .map(|mut declared| {
            declared.retain(|name, _| host_names.iter().any(|host_name| host_name == name));
            out_dir["configurations"] = Value::Table(declared);
        })
}

fn host_out_dirs(
    out_dirs: &mut Value,
    host_names: &[String],
    manifest_path: &Path,
) -> Result<(), HeisenbergCheckError> {
    out_dirs
        .clone()
        .try_into::<Table>()
        .map_err(|source| HeisenbergCheckError::FixtureOutDirs {
            path: manifest_path.to_path_buf(),
            source,
        })
        .and_then(|mut declared| {
            declared
                .iter_mut()
                .try_for_each(|(package, out_dir)| {
                    host_out_dir(package, out_dir, host_names, manifest_path)
                })
                .map(|()| *out_dirs = Value::Table(declared))
        })
}

fn host_settings(settings: &mut Value, manifest_path: &Path) -> Result<(), HeisenbergCheckError> {
    settings["configurations"]
        .clone()
        .try_into::<Vec<Table>>()
        .map_err(|source| HeisenbergCheckError::FixtureConfigurations {
            path: manifest_path.to_path_buf(),
            source,
        })
        .and_then(|configurations| {
            configurations
                .into_iter()
                .enumerate()
                .filter_map(|(index, table)| {
                    host_configuration(table, index, manifest_path).transpose()
                })
                .collect::<Result<Vec<HostConfiguration>, HeisenbergCheckError>>()
        })
        .map(|host_configurations| {
            let host_names: Vec<String> = host_configurations
                .iter()
                .map(|HostConfiguration { name, .. }| name.clone())
                .collect();

            settings["configurations"] = Value::Array(
                host_configurations
                    .into_iter()
                    .map(|HostConfiguration { table, .. }| Value::Table(table))
                    .collect(),
            );

            host_names
        })
        .and_then(|host_names| {
            settings.get_mut("out_dirs").map_or(Ok(()), |out_dirs| {
                host_out_dirs(out_dirs, &host_names, manifest_path)
            })
        })
}

pub struct HostOnlyFixture<'fixture> {
    pub directory: &'fixture Path,
    pub name: &'fixture str,
}

impl HostOnlyFixture<'_> {
    /// # Errors
    ///
    /// Returns the [`HeisenbergCheckError`] of the step that cannot copy the fixture into
    /// `directory` or keep only the configurations of the host's target in it.
    pub fn path(&self) -> Result<PathBuf, HeisenbergCheckError> {
        let source = FixtureWorkspace { name: self.name }.path();
        let source_manifest = source.join("Cargo.toml");
        let copy = self.directory.join(self.name);
        let manifest_path = copy.join("Cargo.toml");

        cleared_path(&copy)
            .and_then(|()| copied_directory(&source, &copy))
            .and_then(|()| {
                fs::read_to_string(&manifest_path).map_err(|source| {
                    HeisenbergCheckError::FixtureManifestRead {
                        path: manifest_path.clone(),
                        source,
                    }
                })
            })
            .and_then(|text| {
                text.parse::<Table>()
                    .map_err(|source| HeisenbergCheckError::FixtureManifest {
                        path: manifest_path.clone(),
                        source,
                    })
            })
            .and_then(|mut manifest| {
                manifest
                    .get_mut("workspace")
                    .and_then(|workspace| workspace.get_mut("metadata"))
                    .and_then(|metadata| metadata.get_mut("heisenberg"))
                    .ok_or(HeisenbergCheckError::FixtureSettingsMissing(
                        manifest_path.clone(),
                    ))
                    .and_then(|settings| host_settings(settings, &manifest_path))
                    .and_then(|()| {
                        to_string(&manifest)
                            .map_err(HeisenbergCheckError::FixtureManifestSerialization)
                    })
            })
            .and_then(|text| {
                fs::write(&manifest_path, text).map_err(|source| {
                    HeisenbergCheckError::FixtureManifestWrite {
                        path: manifest_path.clone(),
                        source,
                    }
                })
            })
            .and_then(|()| preserved_timestamp(&source_manifest, &manifest_path))
            .and_then(|()| {
                written_cargo_configuration(
                    &copy,
                    self.directory
                        .join(HOST_ONLY_BUILDS_DIRECTORY)
                        .join(self.name),
                    &source_manifest,
                )
            })
            .map(|()| copy)
    }
}
