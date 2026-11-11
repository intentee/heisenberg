use std::env;
use std::io;
use std::path::Path;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

use heisenberg_project_model::constants::CLIPPY_ARGUMENTS_VARIABLE;
use heisenberg_project_model::constants::CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE;
use heisenberg_project_model::constants::COMPILER_VARIABLES;
use heisenberg_project_model::constants::NEXTEST_RETRIES_VARIABLE;
use heisenberg_project_model::constants::NIGHTLY_FEATURE_VARIABLES;
use heisenberg_project_model::constants::RUSTFLAGS_VARIABLES;

use crate::constants::HOST_SELECTION;
use crate::constants::JOBS_ARGUMENT;
use crate::constants::LLVM_PROFILE_FILE_VARIABLE;
use crate::constants::PATH_VARIABLE;
use crate::constants::RUSTUP_TOOLCHAIN_VARIABLE;
use crate::heisenberg_arguments::HeisenbergArguments;
use crate::worker_count_run::WorkerCountRun;

pub struct BinaryCheck<'check> {
    pub binary: &'check Path,
    pub cargo_home: &'check Path,
    pub subcommand: &'check str,
    pub sysroot: &'check Path,
    pub workspace: &'check Path,
}

impl BinaryCheck<'_> {
    /// # Errors
    ///
    /// Returns the [`io::Error`] that keeps the binary from starting.
    pub fn spawn(&self, arguments: &[&str]) -> io::Result<Child> {
        self.command(arguments)
            .env_clear()
            .env(PATH_VARIABLE, "")
            .envs(env::vars_os().filter(|(name, _)| name == LLVM_PROFILE_FILE_VARIABLE))
            .spawn()
    }

    /// # Errors
    ///
    /// Returns the [`io::Error`] that keeps either run from starting.
    pub fn spawn_for_both_worker_counts(&self, arguments: &[&str]) -> io::Result<WorkerCountRun> {
        self.spawn_with_toolchain_environment(&[&[JOBS_ARGUMENT, "1"], arguments].concat())
            .and_then(|single_worker| {
                self.spawn_with_toolchain_environment(arguments)
                    .map(|default_workers| WorkerCountRun {
                        default_workers,
                        single_worker,
                    })
            })
    }

    /// # Errors
    ///
    /// Returns the [`io::Error`] that keeps the binary from starting.
    pub fn spawn_with_toolchain_environment(&self, arguments: &[&str]) -> io::Result<Child> {
        let mut command = self.command(arguments);

        for variable in COMPILER_VARIABLES
            .into_iter()
            .chain(RUSTFLAGS_VARIABLES)
            .chain(NIGHTLY_FEATURE_VARIABLES)
            .chain([
                CLIPPY_ARGUMENTS_VARIABLE,
                CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE,
                NEXTEST_RETRIES_VARIABLE,
                RUSTUP_TOOLCHAIN_VARIABLE,
            ])
        {
            command.env_remove(variable);
        }

        command.spawn()
    }

    fn command(&self, arguments: &[&str]) -> Command {
        let mut command = Command::new(self.binary);

        command
            .args(
                HeisenbergArguments {
                    cargo_home: self.cargo_home.as_os_str(),
                    subcommand: self.subcommand,
                    sysroot: self.sysroot,
                    workspace: self.workspace,
                }
                .arguments(),
            )
            .args(HOST_SELECTION)
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        command
    }
}
