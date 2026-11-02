use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn unparsable_macro_expansion_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unparsable_macro_expansion",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: UnparsableMacroExpansion - src/lib.rs:9:1 - Macro expansion cannot be parsed\n\
            unparsable_expansion!();\n\
            \n\
            ERROR: UnparsableMacroExpansion - src/lib.rs:17:1 - Macro expansion cannot be parsed\n\
            unparsable_impl!();\n\
            \n\
            ERROR: UnparsableMacroExpansion - src/lib.rs:26:12 - Macro expansion cannot be parsed\n\
            pub struct Rewritten;\n\
            \n\
            ERROR: UnparsableMacroExpansion - src/lib.rs:34:3 - Macro expansion cannot be parsed\n\
            #[derive(unparsable_derive)]\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
