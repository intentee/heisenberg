use std::path::Path;

use cargo::CargoResult;
use cargo::GlobalContext;
use cargo::util::data_structures::HashMap;
use cargo_util_terminal::Shell;

pub struct ConfiguredContext<'context> {
    pub cargo_home: &'context Path,
    pub environment: &'context HashMap<String, String>,
    pub nightly_features_allowed: bool,
    pub workspace_directory: &'context Path,
}

impl ConfiguredContext<'_> {
    /// # Errors
    ///
    /// Returns the cargo error when the cargo configuration cannot be read.
    pub fn with_shell(&self, shell: Shell) -> CargoResult<GlobalContext> {
        let mut context = GlobalContext::new(
            shell,
            self.workspace_directory.to_path_buf(),
            self.cargo_home.to_path_buf(),
        );

        context.set_env(self.environment.clone());
        context.nightly_features_allowed = self.nightly_features_allowed;
        context
            .configure(0, true, None, true, true, true, &None, &[], &[])
            .map(|()| context)
    }
}
