use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn invalid_rustflags_in_the_build_configuration_fail_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "invalid_rustflags_in_the_build_configuration",
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
             error: the `build.rustflags` value of the cargo configuration of `{}` cannot be read\n\
             error: error in {1}: could not load config key `build.rustflags`\n\
             error: invalid configuration for key `build.rustflags`\n\
             expected a string or array of strings, but found a integer for `build.rustflags` in {1}\n",
            workspace.join("Cargo.toml").display(),
            workspace.join(".cargo").join("config.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
