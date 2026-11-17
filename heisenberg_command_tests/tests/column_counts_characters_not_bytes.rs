use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn column_counts_characters_not_bytes() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "column_after_multibyte_characters",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["GlobImport: 1 violation\n\
            Glob imports hide where names come from. Import each name explicitly.\n\
            \n\
            - src/lib.rs:1:33: Glob import\n    \
            const LABEL: &str = \"żółw\"; use std::fmt::*;\n\
            \n\
            ItemOutOfOrder: 1 violation\n\
            Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names. A module that calls a macro_rules! macro of its parent by its bare name compiles only when it is declared after the macro, so declare the module first and hand it the macro by path: re-export the macro after its definition with pub(crate) use, and import it in the module.\n\
            \n\
            - src/lib.rs:1:33: Item belongs before LABEL: imports from vendor crates precede private helpers\n    \
            const LABEL: &str = \"żółw\"; use std::fmt::*;\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
