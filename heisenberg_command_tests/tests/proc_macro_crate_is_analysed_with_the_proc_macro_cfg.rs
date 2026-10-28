use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn proc_macro_crate_is_analysed_with_the_proc_macro_cfg() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "proc_macro_crate_gating_a_cycle_on_its_crate_type",
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
            cycle in proc_macro_crate_gating_a_cycle_on_its_crate_type lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second: crate::first uses crate::second at src/first.rs:4:20\n  \
            crate::second -> crate::first: crate::second uses crate::first at src/second.rs:3:19\n\
            result: 1 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
