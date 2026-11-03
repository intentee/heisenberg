use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn clippy_configuration_directory_in_the_environment_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_in(&BTreeMap::from([(
        OsString::from("CLIPPY_CONF_DIR"),
        OsString::from("clippy"),
    )]))
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `CLIPPY_CONF_DIR` is set in the environment, and Heisenberg cannot account for a clippy configuration outside the workspace\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
