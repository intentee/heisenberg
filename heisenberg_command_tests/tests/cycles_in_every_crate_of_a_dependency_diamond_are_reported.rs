use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn cycles_in_every_crate_of_a_dependency_diamond_are_reported() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cycles_in_a_dependency_diamond",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 4 crate targets, 12 modules\n\
            analysed on x86_64-unknown-linux-gnu: 4 crate targets, 12 modules\n\
            cycle in bottom lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second: crate::first uses crate::second at bottom/src/first.rs:1:5\n  \
            crate::second -> crate::first: crate::second uses crate::first at bottom/src/second.rs:1:5\n\
            cycle in left lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second: crate::first uses crate::second at left/src/first.rs:1:5\n  \
            crate::second -> crate::first: crate::second uses crate::first at left/src/second.rs:1:5\n\
            cycle in right lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second: crate::first uses crate::second at right/src/first.rs:1:5\n  \
            crate::second -> crate::first: crate::second uses crate::first at right/src/second.rs:1:5\n\
            cycle in top lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second: crate::first uses crate::second at top/src/first.rs:1:5\n  \
            crate::second -> crate::first: crate::second uses crate::first at top/src/second.rs:1:5\n\
            result: 4 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
