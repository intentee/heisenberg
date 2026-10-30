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
        Ok(
            "ERROR: ModuleCycle - src/first.rs:4:24 - Parts of crate depend on each other\n    \
            pub fn second() -> crate::second::Second {\n  \
            crate::first -> crate::second:\n    \
            crate::first uses crate::second at src/first.rs:4:24\n    \
            crate::first uses crate::second at src/first.rs:5:9\n  \
            crate::second -> crate::first:\n    \
            crate::second uses crate::first at src/second.rs:4:23\n    \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            ERROR: ModuleCycle - src/fourth.rs:4:23 - Parts of crate depend on each other\n    \
            pub fn third() -> crate::third::Third {\n  \
            crate::fourth -> crate::third:\n    \
            crate::fourth uses crate::third at src/fourth.rs:4:23\n    \
            crate::fourth uses crate::third at src/fourth.rs:5:9\n  \
            crate::third -> crate::fourth:\n    \
            crate::third uses crate::fourth at src/third.rs:4:24\n    \
            crate::third uses crate::fourth at src/third.rs:5:9\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 5 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
