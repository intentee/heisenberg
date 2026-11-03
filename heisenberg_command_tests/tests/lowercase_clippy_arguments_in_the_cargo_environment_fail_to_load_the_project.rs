use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn lowercase_clippy_arguments_in_the_cargo_environment_fail_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "lowercase_clippy_arguments_in_the_cargo_environment",
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
             error: `CLIPPY_ARGS` is set in the `[env]` table of {}, and Heisenberg cannot account for clippy arguments\n",
            workspace.join(".cargo").join("config.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
