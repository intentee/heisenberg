use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn collect_through_a_const_generic_impl_resolves() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "const_generic_receiver_from_another_crate",
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
            host_footer(2, 5).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
