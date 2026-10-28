use std::process::ExitCode;
use std::process::Termination;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

#[test]
fn exit_statuses_follow_the_documented_exit_codes() {
    assert_eq!(
        [
            HeisenbergExitStatus::Compliant,
            HeisenbergExitStatus::Violations,
            HeisenbergExitStatus::Failure,
            HeisenbergExitStatus::ErrorNotReported,
        ]
        .map(Termination::report),
        [0, 1, 3, 4].map(ExitCode::from)
    );
}
