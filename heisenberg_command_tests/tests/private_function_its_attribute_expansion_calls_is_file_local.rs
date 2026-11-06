use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn private_function_its_attribute_expansion_calls_is_file_local() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "function_under_a_procedural_attribute",
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
            host_footer(2, 4).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
