use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unparsable_expansion_of_a_call_a_dependency_macro_writes_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unparsable_expansion_of_a_dependency_macro_call",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnparsableMacroExpansion: 1 violation\n\
            A macro expansion that does not parse cannot be analysed. Fix the macro so its output is valid Rust.\n\
            \n\
            - src/lib.rs:1:1: Macro expansion cannot be parsed\n    \
            macros::wrapping!();\n      \
            in the expansion of wrapping called at src/lib.rs:1:1\n      \
            in the expansion of unparsable called at macros/src/lib.rs:10:9\n      \
            syntax error at src/lib.rs:1:1: expected `;`, `{`, or `(`\n      \
            syntax error at src/lib.rs:1:1: expected a name\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
