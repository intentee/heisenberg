use std::path::Path;
use std::path::PathBuf;

use crate::constants::FIXTURE_DIRECTORY;

pub struct FixtureWorkspace<'name> {
    pub name: &'name str,
}

impl FixtureWorkspace<'_> {
    #[must_use]
    pub fn path(&self) -> PathBuf {
        Path::new(FIXTURE_DIRECTORY)
            .join("workspaces")
            .join(self.name)
    }
}
