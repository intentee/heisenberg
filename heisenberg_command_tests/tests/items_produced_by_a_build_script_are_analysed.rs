use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

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
        Ok("ModuleNotNamedAfterItsItem: 2 violations\n\
            A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the snake case of its public item.\n\
            \n\
            - src/generated_module.rs:1:12: Module is named generated_module, but its public item names it generated\n    \
            pub struct Generated;\n\
            - src/lib.rs:6:16: Module is named configured, but its public item names it item\n        \
            pub struct Item;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
