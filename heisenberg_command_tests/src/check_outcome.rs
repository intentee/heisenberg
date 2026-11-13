use std::collections::BTreeMap;
use std::ffi::OsString;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

use crate::heisenberg_check_error::HeisenbergCheckError;

pub struct CheckOutcome {
    pub errors: Vec<u8>,
    pub output: Vec<u8>,
    pub status: HeisenbergExitStatus,
}

impl CheckOutcome {
    /// # Errors
    ///
    /// Returns [`HeisenbergCheckError::ErrorReport`] when the command cannot report the errors
    /// of its run.
    pub fn of(
        command: &HeisenbergCommand,
        environment: &BTreeMap<OsString, OsString>,
    ) -> Result<Self, HeisenbergCheckError> {
        let mut errors = Vec::new();
        let mut output = Vec::new();

        command
            .run(environment, &mut output, &mut errors)
            .map_err(HeisenbergCheckError::ErrorReport)
            .map(|status| Self {
                errors,
                output,
                status,
            })
    }
}
