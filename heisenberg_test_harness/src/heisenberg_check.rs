use std::env;
use std::ffi::OsString;
use std::path::Path;

use clap::Error;
use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

use crate::check_outcome::CheckOutcome;

pub struct HeisenbergCheck<'check> {
    pub workspace: &'check Path,
}

impl HeisenbergCheck<'_> {
    #[expect(
        clippy::expect_used,
        reason = "tests run only through the Makefile, which exports the variable"
    )]
    pub fn command(&self, sysroot: &Path) -> Result<HeisenbergCommand, Error> {
        let cargo_home =
            env::var_os("HEISENBERG_CARGO_HOME").expect("HEISENBERG_CARGO_HOME must be exported");
        let mut arguments: Vec<OsString> = ["heisenberg", "check", "--cargo-home"]
            .into_iter()
            .map(OsString::from)
            .collect();

        arguments.push(cargo_home);
        arguments.push(OsString::from("--sysroot"));
        arguments.push(sysroot.as_os_str().to_owned());
        arguments.push(OsString::from("--workspace"));
        arguments.push(self.workspace.as_os_str().to_owned());

        HeisenbergCommand::try_parse_from(arguments)
    }

    pub fn run(&self) -> Result<CheckOutcome, Error> {
        self.toolchain_command()
            .map(|command| CheckOutcome::of(&command))
    }

    #[expect(
        clippy::expect_used,
        reason = "tests run only through the Makefile, which exports the variable"
    )]
    pub fn toolchain_command(&self) -> Result<HeisenbergCommand, Error> {
        self.command(Path::new(
            &env::var_os("HEISENBERG_SYSROOT").expect("HEISENBERG_SYSROOT must be exported"),
        ))
    }
}
