use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn rustflags_in_the_environment_fail_to_load_the_project() {
    for variable in ["CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS"] {
        let outcome = HeisenbergCheck {
            workspace: &FixtureWorkspace {
                name: "acyclic_modules",
            }
            .path(),
        }
        .run_in(&BTreeMap::from([(
            OsString::from(variable),
            OsString::from("--cfg=probe"),
        )]))
        .expect("the arguments must parse");

        assert_eq!(
            str::from_utf8(&outcome.errors),
            Ok(format!(
                "error: the project cannot be loaded\n\
                 error: `{variable}` is set in the environment, and Heisenberg cannot account for rustflags\n"
            )
            .as_str())
        );
        assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
    }
}
