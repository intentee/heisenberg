use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn rustdoc_configured_for_cargo_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_in(&BTreeMap::from([(
        OsString::from("CARGO_BUILD_RUSTDOC"),
        OsString::from("rustdoc"),
    )]))
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `build.rustdoc` is set in environment variable `CARGO_BUILD_RUSTDOC`, and Heisenberg cannot account for the rustdoc that cargo names to build scripts\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
