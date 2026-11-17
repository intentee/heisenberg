use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn ignored_test_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "ignored_tests",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "IgnoredTest: 2 violations\n\
            A skipped test looks like part of the suite while it verifies nothing. Make the test pass, or remove it together with what it was meant to verify.\n\
            \n\
            - src/lib.rs:10:7: Test skipped by an ignore attribute\n        \
            #[ignore]\n\
            - src/lib.rs:16:22: Test skipped by an ignore attribute\n        \
            #[cfg_attr(test, ignore = \"the answer is not checked\")]\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
