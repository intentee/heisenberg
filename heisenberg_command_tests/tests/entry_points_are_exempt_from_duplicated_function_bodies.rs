use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn entry_points_are_exempt_from_duplicated_function_bodies() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "duplicated_entry_points",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["DuplicatedFunctionBody: 1 violation\n\
            Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.\n\
            \n\
            - alpha/src/lib.rs:1:8: Function body is duplicated\n    \
            pub fn main() -> u8 {\n      \
            duplicate at beta/src/lib.rs:1:8\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(8, 8).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
