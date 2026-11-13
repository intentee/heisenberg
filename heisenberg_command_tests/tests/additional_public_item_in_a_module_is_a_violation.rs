use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn additional_public_item_in_a_module_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "additional_public_item",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["AdditionalPublicItem: 8 violations\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - src/items.rs:9:8: Module already has a public item\n    \
            pub fn second() {}\n\
            - src/items.rs:11:12: Module already has a public item\n    \
            pub static THIRD: u8 = 3;\n\
            - src/items.rs:14:14: Module already has a public item\n    \
            macro_rules! fourth {\n\
            - src/items.rs:18:10: Module already has a public item\n    \
            pub type Fifth = u8;\n\
            - src/items.rs:20:11: Module already has a public item\n    \
            pub macro sixth() {}\n\
            - src/items.rs:23:17: Module already has a public item\n        \
            pub safe fn abs(value: i32) -> i32;\n\
            - src/items.rs:26:11: Module already has a public item\n    \
            pub union Seventh {\n\
            - src/items.rs:33:14: Module already has a public item\n        \
            pub enum AlsoNested {}\n\
            \n\
            ------------------------------\n\
            8 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
