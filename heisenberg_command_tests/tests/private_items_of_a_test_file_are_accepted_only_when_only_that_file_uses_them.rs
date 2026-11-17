use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn private_items_of_a_test_file_are_accepted_only_when_only_that_file_uses_them() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "file_local_test_helpers",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "NonTestItemInTestTarget: 2 violations\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/helper_used_by_a_submodule.rs:3:8: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            struct Shared;\n\
            - tests/unused_helper.rs:1:4: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            fn unused() {}\n\
            \n\
            SplitIntegrationTests: 1 violation\n\
            Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules. Set autotests = false, declare one [[test]] target, and include the other test files as its modules.\n\
            \n\
            - Cargo.toml: Package builds 3 integration test targets with libtest's harness instead of one\n      \
            test \"helper_used_by_a_submodule\"\n      \
            test \"local_helpers\"\n      \
            test \"unused_helper\"\n\
            \n\
            ------------------------------\n\
            3 errors found in 2 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(4, 5).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
