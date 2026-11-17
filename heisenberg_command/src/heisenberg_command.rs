use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::io::Write;

use clap::Parser;

use crate::catalogue_command::CatalogueCommand;
use crate::catalogue_format::CatalogueFormat;
use crate::explain_command::ExplainCommand;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::heisenberg_subcommand::HeisenbergSubcommand;
use crate::rendered_report::RenderedReport;
use crate::rule_catalogue::RuleCatalogue;

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
            HeisenbergSubcommand::Catalogue(CatalogueCommand {
                format: CatalogueFormat::Json,
            }) => RuleCatalogue::new(|_| true)
                .write_json(output)
                .map(|()| HeisenbergExitStatus::Explained)
                .map_err(HeisenbergCommandError::WriteCatalogueDocument),
            HeisenbergSubcommand::Catalogue(CatalogueCommand {
                format: CatalogueFormat::Text,
            }) => written(
                output,
                &RuleCatalogue::new(|_| true).text(),
                HeisenbergExitStatus::Explained,
                HeisenbergCommandError::WriteCatalogue,
            ),
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
                    failure
                        .chain()
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
