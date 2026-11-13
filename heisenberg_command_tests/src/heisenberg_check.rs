use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::Path;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

use crate::check_outcome::CheckOutcome;
use crate::constants::CHECK_SUBCOMMAND;
use crate::constants::HOST_SELECTION;
use crate::heisenberg_check_error::HeisenbergCheckError;
use crate::heisenberg_invocation::HeisenbergInvocation;

pub struct HeisenbergCheck<'check> {
    pub workspace: &'check Path,
}

impl HeisenbergCheck<'_> {
    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::CargoHome`] when `HEISENBERG_CARGO_HOME` cannot be read,
    /// and [`HeisenbergCheckError::Arguments`] when the arguments are rejected.
    pub fn command(&self, sysroot: &Path) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        self.invocation()
            .command(sysroot, HOST_SELECTION.map(OsString::from))
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
            .and_then(|command| CheckOutcome::of(&command, environment))
    }

    /// # Errors
    ///
    /// Returns the errors of [`Self::toolchain_command`], with `arguments` appended to the
    /// check command.
    pub fn run_with_arguments(
        &self,
        arguments: &[&str],
    ) -> Result<CheckOutcome, HeisenbergCheckError> {
        self.invocation().run(arguments.iter().map(OsString::from))
    }

    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::Sysroot`] when `HEISENBERG_SYSROOT` cannot be read, and
    /// the errors of [`Self::command`].
    pub fn toolchain_command(&self) -> Result<HeisenbergCommand, HeisenbergCheckError> {
        self.invocation()
            .toolchain_command(HOST_SELECTION.map(OsString::from))
    }

    const fn invocation(&self) -> HeisenbergInvocation<'_> {
        HeisenbergInvocation {
            subcommand: CHECK_SUBCOMMAND,
            workspace: self.workspace,
        }
    }
}
