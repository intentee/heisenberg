use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn environment_variable_name_with_an_invalid_escape_fails_the_analysis() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "environment_variable_name_with_an_invalid_escape",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the module dependencies cannot be extracted for the configuration `{HOST_CONFIGURATION}`\n\
             error: the name \"UNREADABLE\\q\" of an environment variable cannot be read as a string literal: InvalidEscape\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
