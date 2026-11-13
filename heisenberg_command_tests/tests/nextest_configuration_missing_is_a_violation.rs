use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn nextest_configuration_missing_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "nextest_configuration_missing",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "NextestConfigurationMissing: 1 violation\n\
            Every test finishes within the limits, so that a slow test shows slow code instead of stalling the run. Add a .config/nextest.toml whose default profile sets a slow-timeout within the limits, and make the tests that exceed them faster.\n\
            \n\
            - Cargo.toml: Workspace has no .config/nextest.toml\n\
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
