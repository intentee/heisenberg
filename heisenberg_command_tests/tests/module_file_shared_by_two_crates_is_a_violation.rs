use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn module_file_shared_by_two_crates_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "integration_tests_sharing_a_module_file",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/common/mod.rs: Module file belongs to 2 modules\n      \
            crate::common in integration_tests_sharing_a_module_file test \"first\"\n      \
            crate::common in integration_tests_sharing_a_module_file test \"second\"\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(3, 5).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
