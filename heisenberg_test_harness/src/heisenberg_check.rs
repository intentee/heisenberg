use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::path::Path;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

use crate::check_outcome::CheckOutcome;
use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::SYSROOT_VARIABLE;
use crate::test_harness_error::TestHarnessError;

pub struct HeisenbergCheck<'check> {
    pub workspace: &'check Path,
}

impl HeisenbergCheck<'_> {
    pub fn command(&self, sysroot: &Path) -> Result<HeisenbergCommand, TestHarnessError> {
        env::var(CARGO_HOME_VARIABLE)
            .map_err(TestHarnessError::CargoHome)
            .and_then(|cargo_home| {
                HeisenbergCommand::try_parse_from([
                    OsString::from("heisenberg"),
                    OsString::from("check"),
                    OsString::from("--cargo-home"),
                    OsString::from(cargo_home),
                    OsString::from("--sysroot"),
                    sysroot.as_os_str().to_owned(),
                    OsString::from("--workspace"),
                    self.workspace.as_os_str().to_owned(),
                ])
                .map_err(TestHarnessError::Arguments)
            })
    }

    pub fn run(&self) -> Result<CheckOutcome, TestHarnessError> {
        self.run_in(&BTreeMap::new())
    }

    pub fn run_in(
        &self,
        environment: &BTreeMap<OsString, OsString>,
    ) -> Result<CheckOutcome, TestHarnessError> {
        self.toolchain_command()
            .map(|command| CheckOutcome::of(&command, environment))
    }

    pub fn toolchain_command(&self) -> Result<HeisenbergCommand, TestHarnessError> {
        env::var(SYSROOT_VARIABLE)
            .map_err(TestHarnessError::Sysroot)
            .and_then(|sysroot| self.command(Path::new(&sysroot)))
    }
}
