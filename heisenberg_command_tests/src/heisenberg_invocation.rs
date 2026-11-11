use std::collections::BTreeMap;
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::iter;
use std::path::Path;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

use crate::check_outcome::CheckOutcome;
use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::PROGRAM_NAME;
use crate::constants::SYSROOT_VARIABLE;
use crate::heisenberg_arguments::HeisenbergArguments;
use crate::heisenberg_check_error::HeisenbergCheckError;

pub struct HeisenbergInvocation<'invocation> {
    pub subcommand: &'invocation str,
    pub workspace: &'invocation Path,
}

impl HeisenbergInvocation<'_> {
    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::CargoHome`] when `HEISENBERG_CARGO_HOME` cannot be read,
    /// and [`HeisenbergCheckError::Arguments`] when the arguments are rejected.
    pub fn command(
        &self,
        sysroot: &Path,
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        env::var(CARGO_HOME_VARIABLE)
            .map_err(HeisenbergCheckError::CargoHome)
            .and_then(|cargo_home| {
                HeisenbergCommand::try_parse_from(
                    iter::once(OsString::from(PROGRAM_NAME))
                        .chain(
                            HeisenbergArguments {
                                cargo_home: OsStr::new(&cargo_home),
                                subcommand: self.subcommand,
                                sysroot,
                                workspace: self.workspace,
                            }
                            .arguments(),
                        )
                        .chain(arguments),
                )
                .map_err(HeisenbergCheckError::Arguments)
            })
    }

    /// # Errors
    ///
    /// Returns the errors of [`Self::toolchain_command`].
    pub fn run(
        &self,
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Result<CheckOutcome, HeisenbergCheckError> {
        self.toolchain_command(arguments)
            .map(|command| CheckOutcome::of(&command, &BTreeMap::new()))
    }

    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::Sysroot`] when `HEISENBERG_SYSROOT` cannot be read, and
    /// the errors of [`Self::command`].
    pub fn toolchain_command(
        &self,
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        env::var(SYSROOT_VARIABLE)
            .map_err(HeisenbergCheckError::Sysroot)
            .and_then(|sysroot| self.command(Path::new(&sysroot), arguments))
    }
}
