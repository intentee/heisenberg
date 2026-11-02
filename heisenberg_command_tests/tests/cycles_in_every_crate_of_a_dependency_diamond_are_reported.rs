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
        Ok("ModuleCycle: 4 violations\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - bottom/src/first.rs:1:5: Parts of crate depend on each other\n    \
            use crate::second::Second;\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at bottom/src/first.rs:1:5\n        \
            crate::first uses crate::second at bottom/src/first.rs:4:28\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at bottom/src/second.rs:1:5\n        \
            crate::second uses crate::first at bottom/src/second.rs:4:27\n\
            - left/src/first.rs:1:5: Parts of crate depend on each other\n    \
            use crate::second::Second;\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at left/src/first.rs:1:5\n        \
            crate::first uses crate::second at left/src/first.rs:5:28\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at left/src/second.rs:1:5\n        \
            crate::second uses crate::first at left/src/second.rs:4:27\n\
            - right/src/first.rs:1:5: Parts of crate depend on each other\n    \
            use crate::second::Second;\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at right/src/first.rs:1:5\n        \
            crate::first uses crate::second at right/src/first.rs:5:28\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at right/src/second.rs:1:5\n        \
            crate::second uses crate::first at right/src/second.rs:4:27\n\
            - top/src/first.rs:1:5: Parts of crate depend on each other\n    \
            use crate::second::Second;\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at top/src/first.rs:1:5\n        \
            crate::first uses crate::second at top/src/first.rs:6:28\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at top/src/second.rs:1:5\n        \
            crate::second uses crate::first at top/src/second.rs:4:27\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 4 crate targets, 12 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 4 crate targets, 12 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
