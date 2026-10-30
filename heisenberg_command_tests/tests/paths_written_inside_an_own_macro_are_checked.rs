use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: CodeActiveOnNoTarget - src/lib.rs:1:1 - Code is active on no supported target\n\
            #[cfg(generated)]\n\
            \n\
            ERROR: UnresolvedPath - src/lib.rs:13:17 - Path is unresolved\n    \
            let _item = configured_item!();\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
