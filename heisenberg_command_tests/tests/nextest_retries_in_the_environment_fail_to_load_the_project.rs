use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn nextest_retries_in_the_environment_fail_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_in(&BTreeMap::from([(
        OsString::from("NEXTEST_RETRIES"),
        OsString::from("2"),
    )]))
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `NEXTEST_RETRIES` is set in the environment, and Heisenberg cannot account for tests that nextest retries\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
