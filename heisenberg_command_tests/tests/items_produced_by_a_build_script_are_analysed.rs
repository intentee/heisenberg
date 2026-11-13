use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn items_produced_by_a_build_script_are_analysed() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_generated_items",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ModuleNotNamedAfterItsItem: 2 violations\n\
            A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the words of its public item, keeping each word whole and joining it to the next by an underscore or by nothing, or rename the item.\n\
            \n\
            - src/generated_module.rs:1:12: Module is named generated_module, but its public item Generated names it generated\n    \
            pub struct Generated;\n\
            - src/lib.rs:23:16: Module is named configured, but its public item Item names it item\n        \
            pub struct Item;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
