use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn rustflags_in_the_build_configuration_fail_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "rustflags_in_the_build_configuration",
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
             error: `build.rustflags` is set in {}, and Heisenberg cannot account for rustflags\n",
            workspace.join(".cargo").join("config.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
