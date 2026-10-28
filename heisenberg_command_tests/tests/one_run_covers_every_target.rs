use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn one_run_covers_every_target() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cycles_on_every_target_and_on_one",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 5 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            cycle in cycles_on_every_target_and_on_one lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::first -> crate::second: crate::first uses crate::second at src/first.rs:3:20\n  \
            crate::second -> crate::first: crate::second uses crate::first at src/second.rs:3:19\n\
            cycle in cycles_on_every_target_and_on_one lib among the parts of crate (targets: x86_64-pc-windows-msvc):\n  \
            crate::fourth -> crate::third: crate::fourth uses crate::third at src/fourth.rs:3:19\n  \
            crate::third -> crate::fourth: crate::third uses crate::fourth at src/third.rs:3:20\n\
            result: 2 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
