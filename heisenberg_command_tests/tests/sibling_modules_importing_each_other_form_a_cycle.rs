use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn sibling_modules_importing_each_other_form_a_cycle() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "sibling_modules_importing_each_other",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: ModuleCycle - src/first.rs:1:5 - Parts of crate depend on each other\n\
            use crate::second::Second;\n  \
            crate::first -> crate::second:\n    \
            crate::first uses crate::second at src/first.rs:1:5\n    \
            crate::first uses crate::second at src/first.rs:4:28\n  \
            crate::second -> crate::first:\n    \
            crate::second uses crate::first at src/second.rs:1:5\n    \
            crate::second uses crate::first at src/second.rs:4:27\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
