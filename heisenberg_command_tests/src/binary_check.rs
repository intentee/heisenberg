use std::env;
use std::io;
use std::path::Path;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

use crate::constants::HOST_SELECTION;
use crate::constants::LLVM_PROFILE_FILE_VARIABLE;
use crate::constants::PATH_VARIABLE;

pub struct BinaryCheck<'check> {
    pub binary: &'check Path,
    pub cargo_home: &'check Path,
    pub sysroot: &'check Path,
    pub workspace: &'check Path,
}

impl BinaryCheck<'_> {
    /// # Errors
    ///
    /// Returns the [`io::Error`] that keeps the binary from starting.
    pub fn spawn(&self, arguments: &[&str]) -> io::Result<Child> {
        Command::new(self.binary)
            .env_clear()
            .env(PATH_VARIABLE, "")
            .envs(env::vars_os().filter(|(name, _)| name == LLVM_PROFILE_FILE_VARIABLE))
            .arg("check")
            .args(HOST_SELECTION)
            .args(arguments)
            .arg("--cargo-home")
            .arg(self.cargo_home)
            .arg("--sysroot")
            .arg(self.sysroot)
            .arg("--workspace")
            .arg(self.workspace)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }
}
