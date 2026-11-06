use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn failed_macro_expansion_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "failed_macro_expansion",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["FailedMacroExpansion: 2 violations\n\
            A macro call that fails to expand produces no code to analyse. Fix the call so it expands.\n\
            \n\
            - src/lib.rs:1:1: Macro expansion failed\n    \
            include!(\"missing.rs\");\n      \
            expansion failing at src/lib.rs:1:10\n\
            - src/lib.rs:3:3: Macro expansion failed\n    \
            #[derive(core::marker::CoerceShared)]\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
