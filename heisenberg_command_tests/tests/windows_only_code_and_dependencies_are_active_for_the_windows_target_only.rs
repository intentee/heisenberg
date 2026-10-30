use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn windows_only_code_and_dependencies_are_active_for_the_windows_target_only() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "target_gated_cycle",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 2 crate targets, 4 modules\n\
            analysed on x86_64-unknown-linux-gnu: 2 crate targets, 4 modules\n\
            cycle in target_gated_cycle lib among the parts of crate (targets: x86_64-pc-windows-msvc):\n  \
            crate::first -> crate::second:\n    \
            crate::first uses crate::second at src/first.rs:3:25\n  \
            crate::second -> crate::first:\n    \
            crate::second uses crate::first at src/second.rs:6:9\n\
            result: 1 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
