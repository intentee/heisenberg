use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 2;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 3;

#[test]
fn code_active_in_no_configuration_is_not_evaluated_in_a_partial_run() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "code_active_in_no_configuration",
        }
        .path(),
    }
    .run_with_arguments(&[CONFIGURATION_ARGUMENT, HOST_CONFIGURATION])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            {footer}",
            footer = NextestPresenceEvaluation::NotEvaluated.host_footer(1, HOST_MODULES)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
