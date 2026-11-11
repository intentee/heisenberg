use std::ffi::OsString;
use std::path::Path;

use crate::check_outcome::CheckOutcome;
use crate::constants::COVERAGE_CHECK_SUBCOMMAND;
use crate::constants::COVERAGE_EXPORT_ARGUMENT;
use crate::constants::HOST_SELECTION;
use crate::heisenberg_check_error::HeisenbergCheckError;
use crate::heisenberg_invocation::HeisenbergInvocation;

pub struct HeisenbergCoverageCheck<'check> {
    pub export: &'check Path,
    pub workspace: &'check Path,
}

impl HeisenbergCoverageCheck<'_> {
    /// # Errors
    ///
    /// Returns the errors of [`HeisenbergInvocation::run`].
    pub fn run(&self) -> Result<CheckOutcome, HeisenbergCheckError> {
        HeisenbergInvocation {
            subcommand: COVERAGE_CHECK_SUBCOMMAND,
            workspace: self.workspace,
        }
        .run(
            HOST_SELECTION
                .into_iter()
                .chain([COVERAGE_EXPORT_ARGUMENT])
                .map(OsString::from)
                .chain([self.export.as_os_str().to_owned()]),
        )
    }
}
