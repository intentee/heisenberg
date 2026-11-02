use std::env;
use std::io;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

fn main() -> HeisenbergExitStatus {
    HeisenbergCommand::parse().run(
        &env::vars_os().collect(),
        &mut io::stdout().lock(),
        &mut io::stderr().lock(),
    )
}
