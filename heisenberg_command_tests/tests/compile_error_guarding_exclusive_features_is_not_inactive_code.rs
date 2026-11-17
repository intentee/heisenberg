use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::FULL_RUN_FOOTER_END;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn compile_error_guarding_exclusive_features_is_not_inactive_code() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "compile_error_guarding_exclusive_features",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run_with_arguments(&[
        CONFIGURATION_ARGUMENT,
        &format!("{HOST_CONFIGURATION}-shared"),
        CONFIGURATION_ARGUMENT,
        &format!("{HOST_CONFIGURATION}-static"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("CodeActiveInNoConfiguration: 1 violation\n\
            Code active in no declared configuration is never built or analysed. Remove it, declare a configuration it is active in, or make it active in a declared configuration.\n\
            \n\
            - src/lib.rs:7:1: Code is active in no declared configuration\n    \
            std::thread_local! {{\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}-shared: 1 crate targets, 2 modules\n\
            Analysed on {HOST_CONFIGURATION}-static: 1 crate targets, 2 modules\n\
            {FULL_RUN_FOOTER_END}").as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
