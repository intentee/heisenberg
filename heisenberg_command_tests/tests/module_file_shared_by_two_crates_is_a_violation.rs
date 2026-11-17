use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn module_file_shared_by_two_crates_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "integration_tests_sharing_a_module_file",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/common/mod.rs: Module file belongs to 2 modules\n      \
            crate::common in integration_tests_sharing_a_module_file test \"first\"\n      \
            crate::common in integration_tests_sharing_a_module_file test \"second\"\n\
            \n\
            SplitIntegrationTests: 1 violation\n\
            Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules. Set autotests = false, declare one [[test]] target, and include the other test files as its modules.\n\
            \n\
            - Cargo.toml: Package builds 2 integration test targets with libtest's harness instead of one\n      \
            test \"first\"\n      \
            test \"second\"\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(3, 5).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
