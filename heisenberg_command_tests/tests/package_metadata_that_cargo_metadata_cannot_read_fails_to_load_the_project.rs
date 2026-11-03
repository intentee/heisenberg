use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn package_metadata_that_cargo_metadata_cannot_read_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "rust_version_without_a_minor_version",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: cargo_metadata cannot read the metadata cargo produces for package `rust_version_without_a_minor_version v0.1.0 ({})`\n\
             error: unexpected end of input while parsing major version number\n",
            workspace.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
