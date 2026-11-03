use std::path::Path;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::PACKAGE_DIRECTORY;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn heisenberg_workspace_is_compliant() {
    let outcome = HeisenbergCheck {
        workspace: Path::new(PACKAGE_DIRECTORY)
            .parent()
            .expect("the tests package lives inside the Heisenberg workspace"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(outcome.errors, b"");
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
