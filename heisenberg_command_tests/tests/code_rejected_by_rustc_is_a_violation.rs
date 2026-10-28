use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn code_rejected_by_rustc_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "code_rejected_by_rustc",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            violation in code_rejected_by_rustc lib: extern crate is unresolved at src/lib.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in code_rejected_by_rustc lib: macro definition is invalid at src/lib.rs:3:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in code_rejected_by_rustc lib: macro expansion cannot be parsed at src/lib.rs:13:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in code_rejected_by_rustc lib: built-in macro is not implemented at src/lib.rs:16:14 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            result: 0 cycles, 4 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
