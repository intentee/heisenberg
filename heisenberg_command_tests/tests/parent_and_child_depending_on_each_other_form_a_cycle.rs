use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn parent_and_child_depending_on_each_other_form_a_cycle() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "parent_and_child_depending_on_each_other",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: ModuleCycle - src/child.rs:4:24 - Parts of crate depend on each other\n    \
            pub fn parent() -> crate::Parent {\n  \
            crate (own code) -> crate::child:\n    \
            crate uses crate::child at src/lib.rs:6:23\n    \
            crate uses crate::child at src/lib.rs:7:9\n  \
            crate::child -> crate (own code):\n    \
            crate::child uses crate at src/child.rs:4:24\n    \
            crate::child uses crate at src/child.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
