use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn malformed_cargo_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "malformed_cargo_configuration",
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
             error: the cargo configuration of `{}` cannot be loaded\n\
             error: could not load Cargo configuration\n\
             error: could not parse TOML configuration in `{}`\n\
             error: TOML parse error at line 1, column 7\n  \
             |\n\
             1 | [build\n  \
             |       ^\n\
             unclosed table, expected `]`\n",
            workspace.join("Cargo.toml").display(),
            workspace.join(".cargo").join("config.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
