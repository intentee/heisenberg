use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn entry_point_takes_the_place_of_the_public_item_in_the_item_order() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "entry_point_among_items",
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
            - src/main.rs:9:4: Item belongs before Greeting: private helpers precede private types\n    \
            fn announce() {}\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
