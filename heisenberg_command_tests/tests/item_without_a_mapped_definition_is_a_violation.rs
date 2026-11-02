use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("UnmappedItem: 7 violations\n\
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
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
