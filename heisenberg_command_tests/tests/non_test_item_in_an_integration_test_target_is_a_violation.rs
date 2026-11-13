use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn non_test_item_in_an_integration_test_target_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "non_test_item_in_test_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["IgnoredTest: 1 violation\n\
            A skipped test looks like part of the suite while it verifies nothing. Make the test pass, or remove it together with what it was meant to verify.\n\
            \n\
            - tests/mixed.rs:17:3: Test skipped by an ignore attribute\n    \
            #[ignore = \"the scenario needs no run\"]\n\
            \n\
            NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/mixed.rs:21:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n        \
            pub fn helpers() -> super::Library {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(2, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
