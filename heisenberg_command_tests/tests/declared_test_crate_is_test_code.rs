use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["ItemUsedOnlyByTests: 1 violation\n\
            Production code holds only what production uses. Move an item that only tests use into the test module that uses it, or into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - shared/src/for_tests.rs:1:14: Function is used only by tests\n    \
            pub const fn for_tests() -> u8 {\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(7, 9).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
