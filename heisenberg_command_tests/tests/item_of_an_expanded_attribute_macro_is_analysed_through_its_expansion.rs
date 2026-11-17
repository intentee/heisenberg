use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_of_an_expanded_attribute_macro_is_analysed_through_its_expansion() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "attribute_macro_item_analysed_through_its_expansion",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["AdditionalPublicItem: 1 violation\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - src/allocator.rs:6:8: Module already has a public item\n    \
            pub fn second() {}\n\
            \n\
            PublicAssociatedConstant: 1 violation\n\
            Public constants have a single home, crate::constants. Move the constant there as a free constant.\n\
            \n\
            - src/limits.rs:5:15: Public associated constant belongs in crate::constants as a free constant\n        \
            pub const MAXIMUM: u8 = 9;\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
