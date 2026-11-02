use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: AdditionalPublicItem - src/allocator.rs:6:8 - Module already has a public item\n\
            pub fn second() {}\n\
            \n\
            ERROR: PublicAssociatedConstant - src/limits.rs:5:15 - Public associated constant belongs in crate::constants as a free constant\n    \
            pub const MAXIMUM: u8 = 9;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
