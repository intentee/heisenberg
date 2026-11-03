use std::path::Path;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::constants::FIXTURE_DIRECTORY;

#[test]
fn heisenberg_workspace_is_compliant() {
    let outcome = HeisenbergCheck {
        workspace: Path::new(FIXTURE_DIRECTORY)
            .parent()
            .expect("the fixtures crate lives inside the Heisenberg workspace"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(outcome.errors, b"");
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
