use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: AdditionalPublicItem - src/items.rs:3:1 - Module already has a public item\n\
            pub fn second() {}\n\
            \n\
            ERROR: AdditionalPublicItem - src/items.rs:5:1 - Module already has a public item\n\
            pub static THIRD: u8 = 3;\n\
            \n\
            ERROR: AdditionalPublicItem - src/items.rs:7:1 - Module already has a public item\n\
            #[macro_export]\n\
            \n\
            ERROR: AdditionalPublicItem - src/items.rs:21:5 - Module already has a public item\n    \
            pub enum AlsoNested {}\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
