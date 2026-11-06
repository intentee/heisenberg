use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn path_into_a_dependency_module_its_build_script_configures_resolves() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dependency_build_script_cfg",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n",
            host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
