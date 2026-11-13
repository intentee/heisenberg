use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["AdditionalPublicItem: 1 violation\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - tests/cycle.rs:16:12: Module already has a public item\n        \
            pub fn back() {\n\
            \n\
            ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - tests/cycle.rs:9:9: Parts of crate depend on each other\n            \
            super::second::second();\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at tests/cycle.rs:9:9\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at tests/cycle.rs:17:9\n\
            \n\
            NonTestItemInTestTarget: 3 violations\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/cycle.rs:8:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n        \
            pub fn first() {\n\
            - tests/cycle.rs:14:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n        \
            pub fn second() {}\n\
            - tests/cycle.rs:16:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n        \
            pub fn back() {\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(2, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
