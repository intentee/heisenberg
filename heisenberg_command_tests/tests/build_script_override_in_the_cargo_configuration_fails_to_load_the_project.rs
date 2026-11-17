use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn build_script_override_in_the_cargo_configuration_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_override_in_the_cargo_configuration",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `target.x86_64-pc-windows-msvc.foreign` replaces the build script of a package in the cargo configuration, and Heisenberg analyses what the build script emits\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
