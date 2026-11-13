use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_without_a_mapped_definition_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "items_in_type_position_blocks",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnmappedItem: 7 violations\n\
            An item that cannot be mapped to its definition leaves the analysis incomplete. Write it so it maps to a definition.\n\
            \n\
            - src/lib.rs:3:15: Item cannot be mapped to its definition\n            \
            const SIZE: usize = 3;\n\
            - src/lib.rs:7:22: Item cannot be mapped to its definition\n            \
            extern crate core as base;\n\
            - src/lib.rs:11:16: Item cannot be mapped to its definition\n            \
            struct Count;\n\
            - src/lib.rs:12:14: Item cannot be mapped to its definition\n            \
            impl Count {\n\
            - src/lib.rs:18:13: Item cannot be mapped to its definition\n            \
            mod widths {\n\
            - src/lib.rs:26:18: Item cannot be mapped to its definition\n            \
            const fn total(extra: usize) -> usize {\n\
            - src/lib.rs:33:15: Item cannot be mapped to its definition\n            \
            trait Measured {\n\
            \n\
            ------------------------------\n\
            7 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
