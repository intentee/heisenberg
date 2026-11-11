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
use crate::rendered_report::RenderedReport;

fn written(
    output: &mut dyn Write,
    text: &str,
    status: HeisenbergExitStatus,
    failure: fn(io::Error) -> HeisenbergCommandError,
) -> Result<HeisenbergExitStatus, Vec<HeisenbergCommandError>> {
    output
        .write_all(text.as_bytes())
        .map(|()| status)
        .map_err(|error| vec![failure(error)])
}

fn reported(
    output: &mut dyn Write,
    report: Result<RenderedReport, Vec<HeisenbergCommandError>>,
) -> Result<HeisenbergExitStatus, Vec<HeisenbergCommandError>> {
    report.and_then(|RenderedReport { status, text }| {
        written(output, &text, status, HeisenbergCommandError::WriteReport)
    })
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
    pub fn run(
        &self,
        environment: &BTreeMap<OsString, OsString>,
        output: &mut dyn Write,
        errors: &mut dyn Write,
    ) -> HeisenbergExitStatus {
        let outcome = match &self.subcommand {
            HeisenbergSubcommand::Check(check) => reported(
                output,
                check.execute(environment).map(|report| report.render()),
            ),
            HeisenbergSubcommand::CoverageCheck(coverage) => reported(
                output,
                coverage.execute(environment).map(|report| report.render()),
            ),
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
