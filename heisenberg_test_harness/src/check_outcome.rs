use std::collections::BTreeMap;
use std::ffi::OsString;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

pub struct CheckOutcome {
    pub errors: Vec<u8>,
    pub output: Vec<u8>,
    pub status: HeisenbergExitStatus,
}

impl CheckOutcome {
    #[must_use]
    pub fn of(command: &HeisenbergCommand, environment: &BTreeMap<OsString, OsString>) -> Self {
        let mut errors = Vec::new();
        let mut output = Vec::new();
        let status = command.run(environment, &mut output, &mut errors);

        Self {
            errors,
            output,
            status,
        }
    }
}
