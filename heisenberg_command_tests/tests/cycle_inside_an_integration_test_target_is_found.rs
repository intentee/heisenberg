use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn cycle_inside_an_integration_test_target_is_found() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cycle_in_an_integration_test_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("AdditionalPublicItem: 1 violation\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - tests/cycle.rs:10:12: Module already has a public item\n        \
            pub fn back() {\n\
            \n\
            ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - tests/cycle.rs:3:9: Parts of crate depend on each other\n            \
            super::second::helper();\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at tests/cycle.rs:3:9\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at tests/cycle.rs:11:9\n\
            \n\
            NonTestItemInTestTarget: 3 violations\n\
            Integration test files hold only tests. Move helpers and fixtures into a library crate.\n\
            \n\
            - tests/cycle.rs:2:12: Integration test target holds an item that is neither a test, a constant nor an import\n        \
            pub fn run() {\n\
            - tests/cycle.rs:8:12: Integration test target holds an item that is neither a test, a constant nor an import\n        \
            pub fn helper() {}\n\
            - tests/cycle.rs:10:12: Integration test target holds an item that is neither a test, a constant nor an import\n        \
            pub fn back() {\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
