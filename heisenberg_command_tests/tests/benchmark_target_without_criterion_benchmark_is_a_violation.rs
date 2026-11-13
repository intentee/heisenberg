use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn benchmark_target_without_criterion_benchmark_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "benchmark_target_without_criterion_benchmark",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["BenchmarkTargetWithoutCriterionBenchmark: 1 violation\n\
            Heisenberg analyses benchmarks through criterion, the benchmark harness whose measurement it understands, so a benchmark measured any other way escapes its rules. Register every benchmark of the target with criterion in the target's own module files.\n\
            \n\
            - benches/measurement.rs:1:1: Benchmark target registers no criterion benchmark\n    \
            fn main() {}\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
