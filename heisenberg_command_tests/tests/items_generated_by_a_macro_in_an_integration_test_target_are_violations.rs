use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn items_generated_by_a_macro_in_an_integration_test_target_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "macro_generated_items_in_a_test_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NonTestItemInTestTarget: 2 violations\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/generating.rs:7:1: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            generated_type!();\n\
            - tests/generating.rs:12:6: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            impl Generated {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
