use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn function_used_only_by_tests_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "functions_used_only_by_tests",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["FunctionUsedOnlyByTests: 1 violation\n\
            Production code holds only what production uses. Move a function that only tests use into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - library/src/counter.rs:6:18: Function is used only by tests\n        \
            pub const fn for_tests() -> Self {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(4, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
