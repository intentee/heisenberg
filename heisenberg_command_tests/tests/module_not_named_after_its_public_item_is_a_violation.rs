use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn module_not_named_after_its_public_item_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "modules_not_named_after_their_public_items",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ModuleNotNamedAfterItsItem: 3 violations\n\
            A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the words of its public item, keeping each word whole and joining it to the next by an underscore or by nothing, or rename the item.\n\
            \n\
            - src/lib.rs:7:12: Module is named helpers, but its public item parse_map names it parse_map\n        \
            pub fn parse_map() -> u8 {\n\
            - src/ministral_3_cluster_params.rs:1:12: Module is named ministral_3_cluster_params, but its public item Ministral3ClusterParams names it ministral3_cluster_params\n    \
            pub struct Ministral3ClusterParams;\n\
            - src/widget.rs:1:12: Module is named widget, but its public item FooBarBaz names it foo_bar_baz\n    \
            pub struct FooBarBaz;\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
