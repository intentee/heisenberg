use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn module_file_repeated_in_one_crate_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_repeated_in_crate",
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
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            violation in module_file_repeated_in_crate lib: module file is another module of the same crate, so it cannot be analysed as this module at src/shared.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation: module file src/shared.rs belongs to 2 modules (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first in module_file_repeated_in_crate lib\n  \
            crate::second in module_file_repeated_in_crate lib\n\
            result: 0 cycles, 2 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
