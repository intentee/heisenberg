use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn items_produced_by_a_build_script_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_generated_items",
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
            violation in build_script_generated_items lib: module file is unresolved at src/lib.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: path is unresolved at src/lib.rs:14:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: macro call is unresolved at src/lib.rs:16:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: path is unresolved at src/lib.rs:16:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: macro expansion failed at src/lib.rs:18:10 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: macro expansion failed at src/lib.rs:18:18 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: macro expansion failed at src/lib.rs:18:23 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: path is unresolved at src/lib.rs:20:18 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in build_script_generated_items lib: path is unresolved at src/lib.rs:21:5 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation: code is active on no supported target at src/lib.rs:4:1\n\
            violation: code is active on no supported target at src/lib.rs:9:1\n\
            result: 0 cycles, 11 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
