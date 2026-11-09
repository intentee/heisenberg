use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn unvetted_clippy_setting_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_settings_not_vetted",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnvettedClippySetting: 2 violations\n\
            A clippy setting can silence or relax the lints that Heisenberg requires, so a clippy configuration holds only the required settings and settings that only add restrictions. Remove the setting.\n\
            \n\
            - clippy.toml: Clippy configuration sets allow-panic-in-tests, which Heisenberg has not vetted\n\
            - clippy.toml: Clippy configuration sets too-many-lines-threshold, which Heisenberg has not vetted\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
