use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn sibling_modules_inside_a_nested_module_form_a_cycle_among_its_parts() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "siblings_inside_a_nested_module",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            cycle in siblings_inside_a_nested_module lib among the parts of crate::outer (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::outer::first -> crate::outer::second: crate::outer::first uses crate::outer::second at src/outer/first.rs:3:20\n  \
            crate::outer::second -> crate::outer::first: crate::outer::second uses crate::outer::first at src/outer/second.rs:3:19\n\
            result: 1 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
