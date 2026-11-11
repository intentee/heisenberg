use heisenberg_command_tests::constants::JOBS_ARGUMENT;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::heisenberg_check_error::HeisenbergCheckError;

#[test]
fn job_count_beyond_the_worker_limit_is_a_usage_error() {
    let error = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_with_arguments(&[JOBS_ARGUMENT, "65536"])
    .err();

    assert!(
        matches!(
            &error,
            Some(HeisenbergCheckError::Arguments(arguments))
                if arguments.exit_code() == 2
                    && arguments.to_string()
                        == "error: invalid value '65536' for '--jobs <JOBS>': number too large to fit in target type\n\n\
                            For more information, try '--help'.\n"
        ),
        "{error:?}"
    );
}
