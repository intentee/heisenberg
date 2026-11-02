use std::env;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use crate::constants::LLVM_PROFILE_FILE_VARIABLE;
use crate::constants::PATH_VARIABLE;
use crate::test_harness_error::TestHarnessError;

pub struct HeisenbergBinary<'binary> {
    pub cargo_home: &'binary Path,
    pub executable: &'binary Path,
    pub sysroot: &'binary Path,
    pub workspace: &'binary Path,
}

impl HeisenbergBinary<'_> {
    /// # Errors
    ///
    /// Returns [`TestHarnessError::BinaryLaunch`] when the executable cannot be started.
    pub fn run(&self) -> Result<Output, TestHarnessError> {
        Command::new(self.executable)
            .env_clear()
            .env(PATH_VARIABLE, "")
            .envs(env::vars_os().filter(|(name, _)| name == LLVM_PROFILE_FILE_VARIABLE))
            .arg("check")
            .arg("--cargo-home")
            .arg(self.cargo_home)
            .arg("--sysroot")
            .arg(self.sysroot)
            .arg("--workspace")
            .arg(self.workspace)
            .output()
            .map_err(TestHarnessError::BinaryLaunch)
    }
}
