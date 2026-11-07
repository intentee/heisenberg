use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use toml::Table;
use toml::Value;
use toml::to_string;

use heisenberg_error_context::error_context::error_context;

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
        .map_err(error_context(
            entry.path(),
            HeisenbergCheckError::FixtureCopy,
        ))
        .and_then(|file_type| {
            if file_type.is_dir() {
                copied_directory(&entry.path(), &copy)
            } else {
                fs::copy(entry.path(), &copy)
                    .map(drop)
                    .map_err(error_context(
                        entry.path(),
                        HeisenbergCheckError::FixtureCopy,
                    ))
                    .and_then(|()| preserved_timestamp(&entry.path(), &copy))
            }
        })
}

fn copied_directory(source: &Path, destination: &Path) -> Result<(), HeisenbergCheckError> {
    fs::create_dir(destination)
        .map_err(error_context(
            destination.to_path_buf(),
            HeisenbergCheckError::FixtureCopy,
        ))
        .and_then(|()| {
            fs::read_dir(source).map_err(error_context(
                source.to_path_buf(),
                HeisenbergCheckError::FixtureCopy,
            ))
        })
        .and_then(|entries| {
            entries.into_iter().try_for_each(|entry| {
                entry
                    .map_err(error_context(
                        source.to_path_buf(),
                        HeisenbergCheckError::FixtureCopy,
                    ))
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
        .map_err(error_context(
            source.to_path_buf(),
            HeisenbergCheckError::FixtureTimestamp,
        ))
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
                .map_err(error_context(
                    file.clone(),
                    HeisenbergCheckError::FixtureCargoConfigurationWrite,
                ))
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

    table["target"]
        .clone()
        .try_into::<String>()
        .map_err(Box::new)
        .map_err(error_context(
            context.clone(),
            HeisenbergCheckError::FixtureConfigurationTarget,
        ))
        .and_then(move |target| {
            table["name"]
                .clone()
                .try_into::<String>()
                .map_err(Box::new)
                .map_err(error_context(
                    context,
                    HeisenbergCheckError::FixtureConfigurationName,
                ))
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
        .map_err(error_context(
            FixtureOutDir {
                manifest_path: manifest_path.to_path_buf(),
                package: package.to_owned(),
            },
            HeisenbergCheckError::FixtureOutDirConfigurations,
        ))
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
        .map_err(error_context(
            manifest_path.to_path_buf(),
            HeisenbergCheckError::FixtureOutDirs,
        ))
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
        .map_err(error_context(
            manifest_path.to_path_buf(),
            HeisenbergCheckError::FixtureConfigurations,
        ))
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

        fs::create_dir_all(&copy)
            .map_err(error_context(
                copy.clone(),
                HeisenbergCheckError::FixtureDirectoryCreate,
            ))
            .and_then(|()| {
                fs::remove_dir_all(&copy).map_err(error_context(
                    copy.clone(),
                    HeisenbergCheckError::FixtureReset,
                ))
            })
            .and_then(|()| copied_directory(&source, &copy))
            .and_then(|()| {
                fs::read_to_string(&manifest_path).map_err(error_context(
                    manifest_path.clone(),
                    HeisenbergCheckError::FixtureManifestRead,
                ))
            })
            .and_then(|text| {
                text.parse::<Table>().map_err(error_context(
                    manifest_path.clone(),
                    HeisenbergCheckError::FixtureManifest,
                ))
            })
            .and_then(|mut manifest| {
                host_settings(
                    &mut manifest["workspace"]["metadata"]["heisenberg"],
                    &manifest_path,
                )
                .and_then(|()| {
                    to_string(&manifest).map_err(HeisenbergCheckError::FixtureManifestSerialization)
                })
            })
            .and_then(|text| {
                fs::write(&manifest_path, text).map_err(error_context(
                    manifest_path.clone(),
                    HeisenbergCheckError::FixtureManifestWrite,
                ))
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
