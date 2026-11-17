use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn procedural_macro_entry_points_are_exempt_only_from_the_rules_rust_forces() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "identity_marker_macros",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemOutOfOrder: 1 violation\n\
            Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names. A module that calls a macro_rules! macro of its parent by its bare name compiles only when it is declared after the macro, so declare the module first and hand it the macro by path: re-export the macro after its definition with pub(crate) use, and import it in the module.\n\
            \n\
            - src/lib.rs:13:4: Item belongs before service: private helpers precede public items\n    \
            fn passthrough(item: TokenStream) -> TokenStream {\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
