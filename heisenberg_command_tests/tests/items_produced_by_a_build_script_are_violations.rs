use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn items_produced_by_a_build_script_are_violations() {
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
        Ok(
            "ERROR: UnresolvedModule - src/lib.rs:1:1 - Module file is unresolved\n\
            #[cfg_attr(generated, path = \"generated_module.rs\")]\n\
            \n\
            ERROR: CodeActiveOnNoTarget - src/lib.rs:4:1 - Code is active on no supported target\n\
            #[cfg(generated)]\n\
            \n\
            ERROR: CodeActiveOnNoTarget - src/lib.rs:9:1 - Code is active on no supported target\n\
            #[cfg(generated)]\n\
            \n\
            ERROR: UnresolvedPath - src/lib.rs:14:5 - Path is unresolved\n\
            use crate::configured::Item;\n\
            \n\
            ERROR: UnresolvedMacroCall - src/lib.rs:16:1 - Macro call is unresolved\n\
            configured_macro!();\n\
            \n\
            ERROR: UnresolvedPath - src/lib.rs:16:1 - Path is unresolved\n\
            configured_macro!();\n\
            \n\
            ERROR: FailedMacroExpansion - src/lib.rs:18:10 - Macro expansion failed\n\
            include!(concat!(env!(\"OUT_DIR\"), \"/included.rs\"));\n\
            \n\
            ERROR: FailedMacroExpansion - src/lib.rs:18:18 - Macro expansion failed\n\
            include!(concat!(env!(\"OUT_DIR\"), \"/included.rs\"));\n\
            \n\
            ERROR: FailedMacroExpansion - src/lib.rs:18:23 - Macro expansion failed\n\
            include!(concat!(env!(\"OUT_DIR\"), \"/included.rs\"));\n\
            \n\
            ERROR: UnresolvedPath - src/lib.rs:20:18 - Path is unresolved\n\
            pub fn item() -> Item {\n\
            \n\
            ERROR: UnresolvedPath - src/lib.rs:21:5 - Path is unresolved\n    \
            crate::configured::Item\n\
            \n\
            ------------------------------\n\
            11 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
