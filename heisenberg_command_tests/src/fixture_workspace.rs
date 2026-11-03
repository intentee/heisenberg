use std::path::Path;
use std::path::PathBuf;

use crate::constants::PACKAGE_DIRECTORY;

pub struct FixtureWorkspace<'name> {
    pub name: &'name str,
}

impl FixtureWorkspace<'_> {
    #[must_use]
    pub fn path(&self) -> PathBuf {
        Path::new(PACKAGE_DIRECTORY)
            .join("workspaces")
            .join(self.name)
    }
}
