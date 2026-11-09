use clap::error::ErrorKind;

use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::heisenberg_check_error::HeisenbergCheckError;

#[test]
fn missing_configuration_argument_is_a_usage_error() {
    let error = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_with_arguments(&[])
    .err();

    assert!(
        matches!(
            &error,
            Some(HeisenbergCheckError::Arguments(arguments))
                if arguments.kind() == ErrorKind::MissingRequiredArgument
                    && arguments.exit_code() == 2
        ),
        "{error:?}"
    );
}
