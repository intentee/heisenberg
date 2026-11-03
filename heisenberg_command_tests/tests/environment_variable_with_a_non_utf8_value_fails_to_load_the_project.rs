use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::non_utf8_os_string::non_utf8_os_string;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn environment_variable_with_a_non_utf8_value_fails_to_load_the_project() {
    let value = non_utf8_os_string();
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_in(&BTreeMap::from([(OsString::from("PROBE"), value.clone())]))
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the value {value:?} of the environment variable `PROBE` is not UTF-8\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
