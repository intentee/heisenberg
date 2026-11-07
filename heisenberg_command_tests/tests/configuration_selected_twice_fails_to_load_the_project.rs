use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn configuration_selected_twice_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "glob_import",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run_with_arguments(&[
        "--configuration",
        "x86_64-unknown-linux-gnu",
        "--configuration",
        "x86_64-unknown-linux-gnu",
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `x86_64-unknown-linux-gnu` is selected twice with --configuration\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
