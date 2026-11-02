use std::collections::BTreeMap;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::check_outcome::CheckOutcome;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
