use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn type_anchored_path_is_not_ambiguous() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "type_anchored_paths",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n",
            host_footer(1, 3).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
