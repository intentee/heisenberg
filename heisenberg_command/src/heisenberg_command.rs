use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::io::Write;

use clap::Parser;

use heisenberg_error_context::error_chain::error_chain;

use crate::explain_command::ExplainCommand;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::heisenberg_subcommand::HeisenbergSubcommand;

fn written<TOutput: Write>(
    output: &mut TOutput,
    text: &str,
    status: HeisenbergExitStatus,
    failure: fn(io::Error) -> HeisenbergCommandError,
) -> Result<HeisenbergExitStatus, Vec<HeisenbergCommandError>> {
    output
        .write_all(text.as_bytes())
        .map(|()| status)
        .map_err(|error| vec![failure(error)])
}

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
        let outcome = match &self.subcommand {
            HeisenbergSubcommand::Check(check) => check.execute(environment).and_then(|report| {
                written(
                    output,
                    &report.render(),
                    report.status(),
                    HeisenbergCommandError::WriteReport,
                )
            }),
            HeisenbergSubcommand::Explain(ExplainCommand { issue_type }) => written(
                output,
                &issue_type.explanation(),
                HeisenbergExitStatus::Explained,
                HeisenbergCommandError::WriteExplanation,
            ),
        };

        match outcome {
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
