use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn patch_in_the_cargo_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "patch_in_the_cargo_configuration",
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
             error: `patch.crates-io.cfg-if` is set in {}, and Heisenberg checks the patches that the workspace manifest declares\n",
            workspace.join(".cargo").join("config.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
