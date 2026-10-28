use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn path_resolved_inside_a_dependency_macro_closes_a_cycle_at_the_call() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dependency_macro_referencing_the_calling_crate",
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
            cycle in dependency_macro_referencing_the_calling_crate lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::caller -> crate::home: crate::caller uses crate::home at src/caller.rs:2:5\n  \
            crate::home -> crate::caller: crate::home uses crate::caller at src/home.rs:4:5\n\
            result: 1 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
