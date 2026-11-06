use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn module_file_repeated_in_one_crate_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_repeated_in_crate",
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
            - src/shared.rs: Module file belongs to 2 modules\n      \
            crate::first in module_file_repeated_in_crate lib\n      \
            crate::second in module_file_repeated_in_crate lib, not analysed: the crate already analyses the file as crate::first\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
