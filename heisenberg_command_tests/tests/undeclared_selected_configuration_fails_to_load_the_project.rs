use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn undeclared_selected_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "glob_import",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run_with_arguments(&[CONFIGURATION_ARGUMENT, "aarch64-apple-darwin"])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: `aarch64-apple-darwin` is selected with --configuration, but `{}` does not declare it; the declared configurations are x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
