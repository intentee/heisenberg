use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn integration_tests_that_cargo_discovers_are_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "discovered_integration_tests",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "SplitIntegrationTests: 1 violation\n\
            Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules. Set autotests = false, declare one [[test]] target, and include the other test files as its modules.\n\
            \n\
            - Cargo.toml: Package builds integration test targets with libtest's harness without setting autotests = false\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(2, 3).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
