use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 3 crate targets, 5 modules\n\
            analysed on x86_64-unknown-linux-gnu: 3 crate targets, 5 modules\n\
            violation: module file tests/common/mod.rs belongs to 2 modules (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::common in integration_tests_sharing_a_module_file test \"first\"\n  \
            crate::common in integration_tests_sharing_a_module_file test \"second\"\n\
            result: 0 cycles, 1 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
