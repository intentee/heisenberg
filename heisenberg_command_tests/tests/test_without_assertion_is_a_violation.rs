use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn test_without_assertion_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tests_without_assertions",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "TestWithoutAssertion: 2 violations\n\
            A test without an assertion passes whenever its code does not panic, so it checks no result. Assert the values the tested code produces.\n\
            \n\
            - src/lib.rs:16:8: Test asserts nothing\n        \
            fn doubling_runs() {\n\
            - src/lib.rs:32:8: Test asserts nothing\n        \
            fn doubling_is_asserted_only_by_a_nested_function() {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
