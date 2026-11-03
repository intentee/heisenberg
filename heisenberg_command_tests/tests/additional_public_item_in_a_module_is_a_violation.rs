use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

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
        Ok("AdditionalPublicItem: 8 violations\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - src/items.rs:3:8: Module already has a public item\n    \
            pub fn second() {}\n\
            - src/items.rs:5:12: Module already has a public item\n    \
            pub static THIRD: u8 = 3;\n\
            - src/items.rs:8:14: Module already has a public item\n    \
            macro_rules! fourth {\n\
            - src/items.rs:21:14: Module already has a public item\n        \
            pub enum AlsoNested {}\n\
            - src/items.rs:24:10: Module already has a public item\n    \
            pub type Fifth = u8;\n\
            - src/items.rs:26:11: Module already has a public item\n    \
            pub macro sixth() {}\n\
            - src/items.rs:29:17: Module already has a public item\n        \
            pub safe fn abs(value: i32) -> i32;\n\
            - src/items.rs:32:11: Module already has a public item\n    \
            pub union Seventh {\n\
            \n\
            ------------------------------\n\
            8 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
