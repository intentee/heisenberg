use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn missing_required_clippy_setting_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "required_clippy_settings_missing",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RequiredClippySettingMissing: 3 violations\n\
            Tests may fail fast with expect and unwrap, and the lints that clippy relaxes for public items by default must hold for them too. Set each required setting to the value it names.\n\
            \n\
            - clippy.toml: Clippy configuration does not set allow-unwrap-in-tests = true\n\
            - nested/clippy.toml: Clippy configuration does not set allow-expect-in-tests = true\n\
            - nested/clippy.toml: Clippy configuration does not set avoid-breaking-exported-api = false\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n", host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
