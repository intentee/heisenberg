use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn sleep_in_test_code_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "sleep_in_tests",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "SleepInTest: 1 violation\n\
            A test that sleeps waits a guessed time instead of the event it needs, so it is slow when the guess is long and flaky when it is short. Wait for the event itself, such as by joining a thread, receiving from a channel or waiting for a child process.\n\
            \n\
            - src/lib.rs:18:9: Test code sleeps\n            \
            thread::sleep(Duration::ZERO);\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
