use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::FULL_RUN_FOOTER_END;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn each_configuration_is_analysed_with_its_own_declared_build_script_output() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "out_dir_per_configuration",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run_with_arguments(&[
        "--configuration",
        HOST_CONFIGURATION,
        "--configuration",
        &format!("{HOST_CONFIGURATION}-accelerated"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "------------------------------\n\
             No errors found!\n\
             ------------------------------\n\
             Analysed on {HOST_CONFIGURATION}: 3 crate targets, 5 modules\n\
             Analysed on {HOST_CONFIGURATION}-accelerated: 3 crate targets, 6 modules\n\
             {FULL_RUN_FOOTER_END}"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
