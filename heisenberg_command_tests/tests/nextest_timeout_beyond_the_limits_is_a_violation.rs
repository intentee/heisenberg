use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn nextest_timeout_beyond_the_limits_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "nextest_timeouts_beyond_the_limits",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NextestTimeoutNotEnforced: 6 violations\n\
            A test that runs longer than the limits shows slow code or a slow test. Make the code and the test faster instead of relaxing the timeout.\n\
            \n\
            - .config/nextest.toml: Profile slow flags a test as slow after 15s, beyond 10s\n\
            - .config/nextest.toml: Profile default sets no slow-timeout, so nextest never terminates a test\n\
            - .config/nextest.toml: Profile periodic never terminates a slow test\n\
            - .config/nextest.toml: Profile unbounded never terminates a slow test\n\
            - .config/nextest.toml: Profile late terminates a test after 3 periods of 10s, beyond 20s\n\
            - .config/nextest.toml: Profile lenient counts a timed out test as passing\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
