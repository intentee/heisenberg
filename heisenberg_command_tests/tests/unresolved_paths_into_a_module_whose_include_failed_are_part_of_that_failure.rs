use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn unresolved_paths_into_a_module_whose_include_failed_are_part_of_that_failure() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "references_into_a_failed_include",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("FailedMacroExpansion: 1 violation\n\
            A macro call that fails to expand produces no code to analyse. Fix the call so it expands.\n\
            \n\
            - bindings_sys/src/lib.rs:1:1: Macro expansion failed\n    \
            include!(concat!(env!(\"OUT_DIR\"), \"/bindings.rs\"));\n      \
            expansion failing at bindings_sys/src/lib.rs:1:23\n      \
            unresolved paths into bindings_sys: 2, the first at bindings/src/lib.rs:2:18\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
