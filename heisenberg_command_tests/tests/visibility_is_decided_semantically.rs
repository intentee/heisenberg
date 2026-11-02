use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn visibility_is_decided_semantically() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "visibility_decided_semantically",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: PublicConstantOutsideConstantsModule - src/hidden.rs:1:11 - Public constant belongs in crate::constants\n\
            pub const HIDDEN: u8 = 2;\n\
            \n\
            ERROR: AdditionalPublicItem - src/items.rs:3:15 - Module already has a public item\n\
            pub(super) fn sibling() {}\n\
            \n\
            ERROR: Reexport - src/items.rs:9:16 - Re-export\n\
            pub(crate) use std::fmt::Write;\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
