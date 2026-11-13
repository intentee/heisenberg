use std::collections::BTreeMap;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn invalid_unstable_configuration_of_the_sysroot_fails_to_load_the_project() {
    let fixture = FixtureWorkspace {
        name: "sysroot_with_an_invalid_unstable_configuration",
    }
    .path();
    let sysroot = fixture.join("sysroot");
    let library = sysroot
        .join("lib")
        .join("rustlib")
        .join("src")
        .join("rust")
        .join("library");
    let outcome = CheckOutcome::of(
        &HeisenbergCheck {
            workspace: &fixture.join("workspace"),
        }
        .command(&sysroot)
        .expect("the arguments must parse"),
        &BTreeMap::new(),
    )
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the cargo configuration of `{}` cannot be loaded\n\
             error: error in {}: `unstable.mtime-on-use` expected true/false, but found a string\n",
            library.join("Cargo.toml").display(),
            library.join(".cargo").join("config.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
