use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn paths_written_inside_an_own_macro_are_checked() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "own_macro_with_an_unresolvable_body",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("CodeActiveOnNoTarget: 1 violation\n\
            Code active on no declared target is never built or analysed. Remove it, or make it active on a declared target.\n\
            \n\
            - src/lib.rs:2:9: Code is active on no declared target\n    \
            pub mod configured {\n\
            \n\
            UnresolvedPath: 1 violation\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/lib.rs:13:17: Path is unresolved\n        \
            let _item = configured_item!();\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
