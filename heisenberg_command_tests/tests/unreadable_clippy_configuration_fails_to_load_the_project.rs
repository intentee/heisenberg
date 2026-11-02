use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn unreadable_clippy_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "clippy_configuration_unreadable",
    }
    .path();
    let configuration = workspace.join("clippy.toml");
    let read_error =
        fs::read_to_string(&configuration).expect_err("the fixture configuration is a directory");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the clippy configuration `{}` cannot be read\n\
             error: {read_error}\n",
            configuration.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
