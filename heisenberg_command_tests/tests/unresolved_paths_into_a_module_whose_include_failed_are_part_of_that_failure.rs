use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["FailedMacroExpansion: 1 violation\n\
            A macro call that fails to expand produces no code to analyse. Fix the call so it expands.\n\
            \n\
            - bindings_sys/src/lib.rs:1:1: Macro expansion failed\n    \
            include!(concat!(env!(\"OUT_DIR\"), \"/bindings.rs\"));\n      \
            expansion failing at bindings_sys/src/lib.rs:1:23\n      \
            unresolved paths into bindings_sys: 2, the first at bindings/src/lib.rs:2:18\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(3, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
