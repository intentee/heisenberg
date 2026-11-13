use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn function_used_only_by_benchmarks_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "function_used_only_by_benchmarks",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["FunctionUsedOnlyByBenchmarks: 1 violation\n\
            Production code holds only what production uses. Move a function that only benchmarks use, or benchmarks and tests, into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - src/lib.rs:2:8: Function is used only by benchmarks, or by benchmarks and tests\n    \
            pub fn sum(limit: u64) -> u64 {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
