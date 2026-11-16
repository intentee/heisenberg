use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::constants::LEFT_OUT_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_out_of_order_after_different_items_in_each_configuration_is_one_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "item_out_of_order_after_configured_items",
        }
        .path(),
    }
    .run_with_arguments(&[
        CONFIGURATION_ARGUMENT,
        HOST_CONFIGURATION,
        CONFIGURATION_ARGUMENT,
        &format!("{HOST_CONFIGURATION}-without-features"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("ItemOutOfOrder: 1 violation\n\
            Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names. A module that calls a macro_rules! macro of its parent by its bare name compiles only when it is declared after the macro, so declare the module first and hand it the macro by path: re-export the macro after its definition with pub(crate) use, and import it in the module.\n\
            \n\
            - src/report.rs:6:5: Item belongs before LABEL: imports from vendor crates precede private helpers (only in item_out_of_order_after_configured_items lib on {HOST_CONFIGURATION})\n    \
            use std::fmt::Debug;\n      \
            Item belongs before TITLE: imports from vendor crates precede private helpers (only in item_out_of_order_after_configured_items lib on {HOST_CONFIGURATION}-without-features)\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}: 1 crate targets, 2 modules\n\
            Analysed on {HOST_CONFIGURATION}-without-features: 1 crate targets, 2 modules\n\
            Not analysed: {LEFT_OUT_CONFIGURATION}, {LEFT_OUT_CONFIGURATION}-without-features\n\
            {}", NextestPresenceEvaluation::NotEvaluated.partial_run_footer_end()).as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
