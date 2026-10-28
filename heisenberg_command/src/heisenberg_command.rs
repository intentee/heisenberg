use std::error::Error;
use std::io::Write;
use std::iter;

use clap::Parser;

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
        output: &mut TOutput,
        errors: &mut TErrors,
    ) -> HeisenbergExitStatus {
        let HeisenbergSubcommand::Check(check) = &self.subcommand;

        match check.execute().and_then(|report| {
            output
                .write_all(report.render().as_bytes())
                .map(|()| report.status())
                .map_err(HeisenbergCommandError::WriteReport)
        }) {
            Ok(status) => status,
            Err(error) => errors
                .write_all(
                    iter::successors(Some(&error as &dyn Error), |cause: &&dyn Error| {
                        (*cause).source()
                    })
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
