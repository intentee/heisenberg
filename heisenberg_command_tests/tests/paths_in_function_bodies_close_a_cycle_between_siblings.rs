use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn paths_in_function_bodies_close_a_cycle_between_siblings() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "function_body_path_into_a_sibling",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:2:5: Parts of crate depend on each other\n        \
            crate::second::Second::helper();\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:2:5\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
