use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::FULL_RUN_FOOTER_END;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn module_file_skipped_in_some_configurations_names_them() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "module_file_skipped_in_some_configurations",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run_with_arguments(&[
        "--configuration",
        &format!("{HOST_CONFIGURATION}-early"),
        "--configuration",
        &format!("{HOST_CONFIGURATION}-late"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - src/shared.rs: Module file belongs to 3 modules\n      \
            crate::first in module_file_skipped_in_some_configurations lib (only on {HOST_CONFIGURATION}-early)\n      \
            crate::second in module_file_skipped_in_some_configurations lib, not analysed on {HOST_CONFIGURATION}-early: the crate already analyses the file as crate::first\n      \
            crate::third in module_file_skipped_in_some_configurations lib (only on {HOST_CONFIGURATION}-late), not analysed: the crate already analyses the file as crate::second\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}-early: 1 crate targets, 3 modules\n\
            Analysed on {HOST_CONFIGURATION}-late: 1 crate targets, 3 modules\n\
            {FULL_RUN_FOOTER_END}").as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
