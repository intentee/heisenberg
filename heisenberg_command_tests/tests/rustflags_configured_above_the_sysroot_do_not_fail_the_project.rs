use std::collections::BTreeMap;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn rustflags_configured_above_the_sysroot_do_not_fail_the_project() {
    let fixture = FixtureWorkspace {
        name: "sysroot_with_a_rustflags_configuration",
    }
    .path();
    let outcome = CheckOutcome::of(
        &HeisenbergCheck {
            workspace: &fixture.join("workspace"),
        }
        .command(&fixture.join("sysroot"))
        .expect("the arguments must parse"),
        &BTreeMap::new(),
    );

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
