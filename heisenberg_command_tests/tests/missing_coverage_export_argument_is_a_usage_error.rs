use std::ffi::OsString;
use std::path::Path;

use clap::error::ErrorKind;

use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::COVERAGE_CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::PACKAGE_DIRECTORY;
use heisenberg_command_tests::heisenberg_check_error::HeisenbergCheckError;
use heisenberg_command_tests::heisenberg_invocation::HeisenbergInvocation;

#[test]
fn missing_coverage_export_argument_is_a_usage_error() {
    let error = HeisenbergInvocation {
        subcommand: COVERAGE_CHECK_SUBCOMMAND,
        workspace: Path::new(PACKAGE_DIRECTORY),
    }
    .command(
        Path::new(PACKAGE_DIRECTORY),
        [CONFIGURATION_ARGUMENT, "first"].map(OsString::from),
    )
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
