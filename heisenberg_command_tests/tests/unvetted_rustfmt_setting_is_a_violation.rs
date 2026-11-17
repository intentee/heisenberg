use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unvetted_rustfmt_setting_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "rustfmt_settings_not_vetted",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnvettedRustfmtSetting: 3 violations\n\
            A rustfmt setting can exempt code from formatting, and an unstable or unknown one is ignored by the supported toolchain. Remove the setting.\n\
            \n\
            - rustfmt.toml: Setting disable_all_formatting is not one that Heisenberg has vetted\n\
            - rustfmt.toml: Setting ignore is not one that Heisenberg has vetted\n\
            - src/nested/.rustfmt.toml: Setting skip_children is not one that Heisenberg has vetted\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 2)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
