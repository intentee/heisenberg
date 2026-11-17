use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unvetted_nextest_setting_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "nextest_settings_not_vetted",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnvettedNextestSetting: 4 violations\n\
            Every test runs and passes on its first attempt within the limits. Remove the setting, and make the tests that needed it faster or reliable.\n\
            \n\
            - .config/nextest.toml: Nextest configuration sets profile.default.default-filter, which Heisenberg has not vetted\n\
            - .config/nextest.toml: Nextest configuration sets profile.default.retries, which Heisenberg has not vetted\n\
            - .config/nextest.toml: Nextest configuration sets profile.default.slow-timeout.grace-period, which Heisenberg has not vetted\n\
            - .config/nextest.toml: Nextest configuration sets store, which Heisenberg has not vetted\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
