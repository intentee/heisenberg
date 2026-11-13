use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_following_an_item_of_a_later_group_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "items_out_of_order",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemOutOfOrder: 10 violations\n\
            Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names.\n\
            \n\
            - app/src/area.rs:2:5: Item belongs before crate::constants::UNIT: imports from workspace members precede imports from this crate\n    \
            use shapes::square::Square;\n\
            - app/src/area.rs:3:5: Item belongs before crate::constants::UNIT: imports from vendor crates precede imports from this crate\n    \
            use std::fmt::Display;\n\
            - app/src/audit.rs:3:8: Item belongs before Audit: private types precede public items\n    \
            struct Trail;\n\
            - app/src/ledger.rs:5:4: Item belongs before Entry: private helpers precede private types\n    \
            fn total(entries: &[Entry]) -> u32 {\n\
            - app/src/ledger.rs:19:6: Item belongs before Ledger: private types precede public items\n    \
            impl Entry {\n\
            - app/src/mixed.rs:1:30: Item belongs before crate::constants::UNIT: imports from vendor crates precede imports from this crate\n    \
            use {crate::constants::UNIT, std::fmt::Display};\n\
            - app/src/nested.rs:3:12: Item belongs before inner: public items precede inline modules\n    \
            pub struct Nested;\n\
            - app/src/report.rs:3:9: Item belongs before std::fmt::Display: module declarations precede imports from vendor crates\n    \
            pub mod summary;\n\
            - app/src/sequence.rs:5:5: Item belongs before width: imports from vendor crates precede private helpers\n    \
            use std::fmt::Display;\n\
            - app/src/variant.rs:5:5: Item belongs before Shape: imports from this crate precede private types\n    \
            use Shape::Circle;\n\
            \n\
            ------------------------------\n\
            10 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 14).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
