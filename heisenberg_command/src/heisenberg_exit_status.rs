use std::process::ExitCode;
use std::process::Termination;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeisenbergExitStatus {
    Compliant,
    ErrorNotReported,
    Failure,
    Violations,
}

impl Termination for HeisenbergExitStatus {
    fn report(self) -> ExitCode {
        ExitCode::from(match self {
            Self::Compliant => 0,
            Self::Violations => 1,
            Self::Failure => 3,
            Self::ErrorNotReported => 4,
        })
    }
}
