use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::path::Path;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

use crate::check_outcome::CheckOutcome;
use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::HOST_SELECTION;
use crate::constants::SYSROOT_VARIABLE;
use crate::heisenberg_check_error::HeisenbergCheckError;

pub struct HeisenbergCheck<'check> {
    pub workspace: &'check Path,
}

impl HeisenbergCheck<'_> {
    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::CargoHome`] when `HEISENBERG_CARGO_HOME` cannot be read,
    /// and [`HeisenbergCheckError::Arguments`] when the arguments are rejected.
    pub fn command(&self, sysroot: &Path) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        self.command_with(sysroot, &HOST_SELECTION)
    }

    /// # Errors
    ///
    /// Returns the errors of [`Self::toolchain_command`].
    pub fn run(&self) -> Result<CheckOutcome, HeisenbergCheckError> {
        self.run_in(&BTreeMap::new())
    }

    /// # Errors
    ///
    /// Returns the errors of [`Self::toolchain_command`].
    pub fn run_in(
        &self,
        environment: &BTreeMap<OsString, OsString>,
    ) -> Result<CheckOutcome, HeisenbergCheckError> {
        self.toolchain_command()
            .map(|command| CheckOutcome::of(&command, environment))
    }

    /// # Errors
    ///
    /// Returns the errors of [`Self::toolchain_command`], with `arguments` appended to the
    /// check command.
    pub fn run_with_arguments(
        &self,
        arguments: &[&str],
    ) -> Result<CheckOutcome, HeisenbergCheckError> {
        self.toolchain_command_with(arguments)
            .map(|command| CheckOutcome::of(&command, &BTreeMap::new()))
    }

    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::Sysroot`] when `HEISENBERG_SYSROOT` cannot be read, and
    /// the errors of [`Self::command`].
    pub fn toolchain_command(&self) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        self.toolchain_command_with(&HOST_SELECTION)
    }

    fn command_with(
        &self,
        sysroot: &Path,
        arguments: &[&str],
    ) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        env::var(CARGO_HOME_VARIABLE)
            .map_err(HeisenbergCheckError::CargoHome)
            .and_then(|cargo_home| {
                HeisenbergCommand::try_parse_from(
                    [
                        OsString::from("heisenberg"),
                        OsString::from("check"),
                        OsString::from("--cargo-home"),
                        OsString::from(cargo_home),
                        OsString::from("--sysroot"),
                        sysroot.as_os_str().to_owned(),
                        OsString::from("--workspace"),
                        self.workspace.as_os_str().to_owned(),
                    ]
                    .into_iter()
                    .chain(arguments.iter().map(OsString::from)),
                )
                .map_err(HeisenbergCheckError::Arguments)
            })
    }

    fn toolchain_command_with(
        &self,
        arguments: &[&str],
    ) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        env::var(SYSROOT_VARIABLE)
            .map_err(HeisenbergCheckError::Sysroot)
            .and_then(|sysroot| self.command_with(Path::new(&sysroot), arguments))
    }
}
