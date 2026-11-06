use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn compiler_configured_for_cargo_fails_to_load_the_project() {
    for (variable, key) in BTreeMap::from([
        ("CARGO_BUILD_RUSTC", "build.rustc"),
        (
            "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER",
            "build.rustc-workspace-wrapper",
        ),
        ("CARGO_BUILD_RUSTC_WRAPPER", "build.rustc-wrapper"),
    ]) {
        let outcome = HeisenbergCheck {
            workspace: &FixtureWorkspace {
                name: "acyclic_modules",
            }
            .path(),
        }
        .run_in(&BTreeMap::from([(
            OsString::from(variable),
            OsString::from("rustc"),
        )]))
        .expect("the arguments must parse");

        assert_eq!(
            str::from_utf8(&outcome.errors),
            Ok(format!(
                "error: the project cannot be loaded\n\
                 error: `{key}` is set in environment variable `{variable}`, and Heisenberg builds with the rustc of the sysroot\n"
            )
            .as_str())
        );
        assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
    }
}
