use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn code_active_on_no_supported_target_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "code_active_on_no_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            violation: code is active on no supported target at src/lib.rs:1:1\n\
            violation: code is active on no supported target at src/never.rs:4:5\n\
            violation: code is active on no supported target at src/windows_only.rs:4:5\n\
            result: 0 cycles, 3 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
