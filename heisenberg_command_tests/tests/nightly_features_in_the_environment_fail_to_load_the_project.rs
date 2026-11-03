use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn nightly_features_in_the_environment_fail_to_load_the_project() {
    for variable in [
        "RUSTC_BOOTSTRAP",
        "__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS",
    ] {
        let outcome = HeisenbergCheck {
            workspace: &FixtureWorkspace {
                name: "acyclic_modules",
            }
            .path(),
        }
        .run_in(&BTreeMap::from([(
            OsString::from(variable),
            OsString::from("nightly"),
        )]))
        .expect("the arguments must parse");

        assert_eq!(
            str::from_utf8(&outcome.errors),
            Ok(format!(
                "error: the project cannot be loaded\n\
                 error: `{variable}` is set in the environment, and Heisenberg cannot account for nightly features\n"
            )
            .as_str())
        );
        assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
    }
}
