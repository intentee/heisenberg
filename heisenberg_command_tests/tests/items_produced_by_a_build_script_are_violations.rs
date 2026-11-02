use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn items_produced_by_a_build_script_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_generated_items",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("CodeActiveOnNoTarget: 2 violations\n\
            Code active on no declared target is never built or analysed. Remove it, or make it active on a declared target.\n\
            \n\
            - src/lib.rs:5:9: Code is active on no declared target\n    \
            pub mod configured {\n\
            - src/lib.rs:10:14: Code is active on no declared target\n    \
            macro_rules! configured_macro {\n\
            \n\
            FailedMacroExpansion: 1 violation\n\
            A macro call that fails to expand produces no code to analyse. Fix the call so it expands.\n\
            \n\
            - src/lib.rs:18:1: Macro expansion failed\n    \
            include!(concat!(env!(\"OUT_DIR\"), \"/included.rs\"));\n      \
            expansion failing at src/lib.rs:18:10\n      \
            expansion failing at src/lib.rs:18:18\n      \
            expansion failing at src/lib.rs:18:23\n\
            \n\
            UnresolvedMacroCall: 1 violation\n\
            Every name must resolve for the analysis to be complete. Define or import the macro.\n\
            \n\
            - src/lib.rs:16:1: Macro call is unresolved\n    \
            configured_macro!();\n\
            \n\
            UnresolvedModule: 1 violation\n\
            Every name must resolve for the analysis to be complete. Create the module file or remove the declaration.\n\
            \n\
            - src/lib.rs:2:9: Module file is unresolved\n    \
            pub mod generated;\n\
            \n\
            UnresolvedPath: 2 violations\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/lib.rs:14:5: Path is unresolved\n    \
            use crate::configured::Item;\n\
            - src/lib.rs:21:5: Path is unresolved\n        \
            crate::configured::Item\n\
            \n\
            ------------------------------\n\
            7 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
