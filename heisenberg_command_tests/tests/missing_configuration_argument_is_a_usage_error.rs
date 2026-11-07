use clap::error::ErrorKind;

use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::heisenberg_check_error::HeisenbergCheckError;

#[test]
fn missing_configuration_argument_is_a_usage_error() {
    let Err(HeisenbergCheckError::Arguments(error)) = (HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    })
    .run_with_arguments(&[]) else {
        panic!("a check without --configuration must be rejected");
    };

    assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
    assert_eq!(error.exit_code(), 2);
}
