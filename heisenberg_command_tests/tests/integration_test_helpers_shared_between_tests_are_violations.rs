use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn integration_test_helpers_shared_between_tests_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "integration_test_helpers",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/support/fixture_value.rs:1:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            pub struct FixtureValue {\n      \
            impl block at tests/support/fixture_value.rs:5:6\n\
            \n\
            SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/support/mod.rs: Module file belongs to 2 modules\n      \
            crate::support in integration_test_helpers test \"first_shared\"\n      \
            crate::support in integration_test_helpers test \"second_shared\"\n      \
            files of its submodules:\n        \
            tests/support/fixture_value.rs\n\
            \n\
            SplitIntegrationTests: 1 violation\n\
            Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules. Set autotests = false, declare one [[test]] target, and include the other test files as its modules.\n\
            \n\
            - Cargo.toml: Package builds 4 integration test targets with libtest's harness instead of one\n      \
            test \"contained_operation\"\n      \
            test \"first_shared\"\n      \
            test \"recorded_values\"\n      \
            test \"second_shared\"\n\
            \n\
            ------------------------------\n\
            3 errors found in 3 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(5, 9).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
