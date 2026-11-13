use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn benchmark_target_with_libtest_harness_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "benchmark_target_with_libtest_harness",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["BenchmarkTargetWithLibtestHarness: 1 violation\n\
            A benchmark target measures through criterion from its own entry point. Declare the target in a [[bench]] table with harness = false and register its benchmarks with criterion, and move its tests into a tests directory.\n\
            \n\
            - Cargo.toml: Benchmark target bench \"measurement\" uses libtest's harness, which runs no benchmark on stable Rust\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
