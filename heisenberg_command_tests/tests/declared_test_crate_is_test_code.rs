use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn declared_test_crate_is_test_code() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "declared_test_crates",
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
            - shared/src/for_tests.rs:1:14: Function is used only by tests\n    \
            pub const fn for_tests() -> u8 {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(7, 9).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
