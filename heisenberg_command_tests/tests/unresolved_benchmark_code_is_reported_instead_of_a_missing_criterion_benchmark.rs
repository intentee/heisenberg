use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unresolved_benchmark_code_is_reported_instead_of_a_missing_criterion_benchmark() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "benchmark_target_with_unresolved_code",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnresolvedMethodCall: 3 violations\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the call so that rust-analyzer resolves its method, such as by naming the type of its receiver.\n\
            \n\
            - benches/measurement.rs:6:15: Method call compiles, but rust-analyzer cannot resolve its method\n        \
            criterion.bench_function(\"sum\", |bencher| {\n\
            - benches/measurement.rs:7:17: Method call compiles, but rust-analyzer cannot resolve its method\n            \
            bencher.iter(|| (0..64_u64).sum::<u64>());\n\
            - benches/measurement.rs:9:15: Method call compiles, but rust-analyzer cannot resolve its method\n        \
            criterion.final_summary();\n\
            \n\
            UnresolvedPath: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - benches/measurement.rs:1:5: Path compiles, but rust-analyzer cannot resolve it\n    \
            use criterion::Criterion;\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
