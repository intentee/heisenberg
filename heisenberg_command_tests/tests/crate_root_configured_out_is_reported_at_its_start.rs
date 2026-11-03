use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn crate_root_configured_out_is_reported_at_its_start() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "crate_root_configured_out",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("CodeActiveOnNoTarget: 1 violation\n\
            Code active on no declared target is never built or analysed. Remove it, or make it active on a declared target.\n\
            \n\
            - src/lib.rs:1:1: Code is active on no declared target\n    \
            #![cfg(any())]\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
