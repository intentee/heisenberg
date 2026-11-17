use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn path_dependency_named_criterion_is_not_the_benchmark_harness() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "path_dependency_named_criterion_is_not_the_harness",
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
            use criterion::Criterion;\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
