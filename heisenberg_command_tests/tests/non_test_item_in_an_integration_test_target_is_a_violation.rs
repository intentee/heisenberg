use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn non_test_item_in_an_integration_test_target_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "non_test_item_in_test_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 2 crate targets, 3 modules\n\
            analysed on x86_64-unknown-linux-gnu: 2 crate targets, 3 modules\n\
            violation in non_test_item_in_test_target test \"mixed\": integration test target holds an item that is neither a test nor an import at tests/mixed.rs:4:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in non_test_item_in_test_target test \"mixed\": integration test target holds an item that is neither a test nor an import at tests/mixed.rs:9:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            result: 0 cycles, 2 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
