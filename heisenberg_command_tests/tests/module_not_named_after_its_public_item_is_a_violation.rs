use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

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
        Ok("ModuleNotNamedAfterItsItem: 2 violations\n\
            A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the snake case of its public item.\n\
            \n\
            - src/lib.rs:6:12: Module is named helpers, but its public item names it parse_map\n        \
            pub fn parse_map() -> u8 {\n\
            - src/widget.rs:1:12: Module is named widget, but its public item names it foo_bar_baz\n    \
            pub struct FooBarBaz;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
