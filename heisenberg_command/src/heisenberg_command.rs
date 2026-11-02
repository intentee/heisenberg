use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::Write;

use clap::Parser;

use heisenberg_error_context::error_chain::error_chain;

use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::heisenberg_subcommand::HeisenbergSubcommand;

#[derive(Debug, Parser)]
#[command(
    about = "Deterministic, zero-heuristics Rust linter",
    name = "heisenberg",
    version
)]
pub struct HeisenbergCommand {
    #[command(subcommand)]
    pub subcommand: HeisenbergSubcommand,
}

impl HeisenbergCommand {
    pub fn run<TOutput: Write, TErrors: Write>(
        &self,
        environment: &BTreeMap<OsString, OsString>,
        output: &mut TOutput,
        errors: &mut TErrors,
    ) -> HeisenbergExitStatus {
        let HeisenbergSubcommand::Check(check) = &self.subcommand;

        match check.execute(environment).and_then(|report| {
            output
                .write_all(report.render().as_bytes())
                .map(|()| report.status())
                .map_err(|error| vec![HeisenbergCommandError::WriteReport(error)])
        }) {
            Ok(status) => status,
            Err(failures) => errors
                .write_all(
                    failures
                        .iter()
                        .flat_map(error_chain)
                        .map(|cause| format!("error: {cause}\n"))
                        .collect::<Vec<String>>()
                        .concat()
                        .as_bytes(),
                )
                .map_or(HeisenbergExitStatus::ErrorNotReported, |()| {
                    HeisenbergExitStatus::Failure
                }),
        }
    }
}
