use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

use toml::Table;
use toml::Value;
use toml::to_string;

use heisenberg_error_context::error_context::error_context;

use crate::constants::HOST_TRIPLE;
use crate::fixture_workspace::FixtureWorkspace;
use crate::heisenberg_check_error::HeisenbergCheckError;

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

pub struct HostOnlyFixture<'fixture> {
    pub directory: &'fixture Path,
    pub name: &'fixture str,
}

impl HostOnlyFixture<'_> {
    /// # Errors
    ///
    /// Returns the [`HeisenbergCheckError`] of the step that cannot copy the fixture into
    /// `directory` or declare the host's target as its only target.
    pub fn path(&self) -> Result<PathBuf, HeisenbergCheckError> {
        let copy = self.directory.join(self.name);
        let manifest_path = copy.join("Cargo.toml");

        fs::create_dir_all(&copy)
            .and_then(|()| fs::remove_dir_all(&copy))
            .and_then(|()| fs::create_dir_all(self.directory))
            .map_err(error_context(
                copy.clone(),
                HeisenbergCheckError::FixtureReset,
            ))
            .and_then(|()| copied_directory(&FixtureWorkspace { name: self.name }.path(), &copy))
            .and_then(|()| {
                fs::read_to_string(&manifest_path).map_err(error_context(
                    manifest_path.clone(),
                    HeisenbergCheckError::FixtureCopy,
                ))
            })
            .and_then(|text| {
                text.parse::<Table>().map_err(error_context(
                    manifest_path.clone(),
                    HeisenbergCheckError::FixtureManifest,
                ))
            })
            .and_then(|mut manifest| {
                manifest["workspace"]["metadata"]["heisenberg"]["targets"] =
                    Value::Array(vec![Value::String(HOST_TRIPLE.to_owned())]);

                to_string(&manifest).map_err(HeisenbergCheckError::FixtureManifestSerialization)
            })
            .and_then(|text| {
                fs::write(&manifest_path, text).map_err(error_context(
                    manifest_path.clone(),
                    HeisenbergCheckError::FixtureManifestWrite,
                ))
            })
            .map(|()| copy)
    }
}
