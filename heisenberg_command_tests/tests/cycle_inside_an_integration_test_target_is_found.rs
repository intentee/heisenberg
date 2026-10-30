use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn cycle_inside_an_integration_test_target_is_found() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cycle_in_an_integration_test_target",
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
            cycle in cycle_in_an_integration_test_target test \"cycle\" among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second:\n    \
            crate::first uses crate::second at tests/cycle.rs:3:9\n  \
            crate::second -> crate::first:\n    \
            crate::second uses crate::first at tests/cycle.rs:11:9\n\
            violation in cycle_in_an_integration_test_target test \"cycle\": integration test target holds an item that is neither a test nor an import at tests/cycle.rs:2:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in cycle_in_an_integration_test_target test \"cycle\": integration test target holds an item that is neither a test nor an import at tests/cycle.rs:8:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in cycle_in_an_integration_test_target test \"cycle\": module already has a public item at tests/cycle.rs:10:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in cycle_in_an_integration_test_target test \"cycle\": integration test target holds an item that is neither a test nor an import at tests/cycle.rs:10:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            result: 1 cycles, 4 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
