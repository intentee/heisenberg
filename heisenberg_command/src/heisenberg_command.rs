use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::io::Write;
use std::iter;

use clap::Parser;

use crate::explain_command::ExplainCommand;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::heisenberg_subcommand::HeisenbergSubcommand;
use crate::rendered_report::RenderedReport;

fn error_chain(error: &HeisenbergCommandError) -> Vec<String> {
    match error {
        HeisenbergCommandError::Failures(failures) => {
            failures.iter().flat_map(error_chain).collect()
        }
        _ => iter::successors(Some(error as &dyn Error), |cause: &&dyn Error| {
            (*cause).source()
        })
        .map(|cause| cause.to_string().trim_end().to_owned())
        .collect(),
    }
}

fn written(
    output: &mut dyn Write,
    text: &str,
    status: HeisenbergExitStatus,
    failure: fn(io::Error) -> HeisenbergCommandError,
) -> Result<HeisenbergExitStatus, HeisenbergCommandError> {
    output
        .write_all(text.as_bytes())
        .map(|()| status)
        .map_err(failure)
}

fn reported(
    output: &mut dyn Write,
    report: Result<RenderedReport, HeisenbergCommandError>,
) -> Result<HeisenbergExitStatus, HeisenbergCommandError> {
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
    /// # Errors
    ///
    /// Returns the [`io::Error`] of writing the errors of a failed run, which the run cannot
    /// report itself.
    pub fn run(
        &self,
        environment: &BTreeMap<OsString, OsString>,
        output: &mut dyn Write,
        errors: &mut dyn Write,
    ) -> Result<HeisenbergExitStatus, io::Error> {
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
            Ok(status) => Ok(status),
            Err(failure) => errors
                .write_all(
                    error_chain(&failure)
                        .iter()
                        .map(|cause| format!("error: {cause}\n"))
                        .collect::<Vec<String>>()
                        .concat()
                        .as_bytes(),
                )
                .map(|()| HeisenbergExitStatus::Failure),
        }
    }
}
