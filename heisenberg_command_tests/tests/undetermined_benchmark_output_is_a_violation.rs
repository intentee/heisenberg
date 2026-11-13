use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn undetermined_benchmark_output_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "undetermined_benchmark_output",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UndeterminedBenchmarkOutput: 2 violations\n\
            Whether criterion drops a must-use value is decided where the routine is measured. Write the measured routine where it is measured, so that the type it returns is concrete.\n\
            \n\
            - benches/measurement.rs:17:13: Measured routine returns TOutput, which leaves undetermined whether criterion drops a must-use value\n        \
            bencher.iter(routine);\n\
            - benches/measurement.rs:21:13: Measured routine returns (u64, TOutput), which leaves undetermined whether criterion drops a must-use value\n        \
            bencher.iter(|| (64_u64, routine()));\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
