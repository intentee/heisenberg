use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unparsable_expansion_names_its_macro_chain_and_syntax_errors() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unparsable_expansion_through_a_macro_chain",
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
            - library/src/lib.rs:13:3: Macro expansion cannot be parsed\n    \
            #[attributes::submitted]\n      \
            in the expansion of submitted called at library/src/lib.rs:13:3\n      \
            in the expansion of submit called at attributes/src/lib.rs:11:8\n      \
            in the expansion of do_submit called at library/src/lib.rs:9:9\n      \
            syntax error at library/src/lib.rs:13:3: expected `;`, `{`, or `(`\n      \
            syntax error at library/src/lib.rs:13:3: expected a name\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
