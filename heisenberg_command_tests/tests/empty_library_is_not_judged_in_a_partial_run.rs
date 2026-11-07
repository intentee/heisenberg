use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn empty_library_is_not_judged_in_a_partial_run() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "library_with_only_inactive_items",
        }
        .path(),
    }
    .run_with_arguments(&["--configuration", HOST_CONFIGURATION])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            {footer}",
            footer = host_footer(3, 3)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
