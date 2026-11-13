use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn must_use_benchmark_output_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "must_use_benchmark_output",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["MustUseBenchmarkOutput: 10 violations\n\
            A must-use value that criterion drops hides an error or work that never ran, so the benchmark can measure a failure path or the construction of lazy work instead of the code. Handle the outcome inside the routine, for example by recording errors and failing the benchmark after measuring, or validate the input before measuring.\n\
            \n\
            - benches/measurement.rs:22:17: Measured routine returns Result<u64, ParseIntError>, a must-use value that criterion drops\n            \
            bencher.iter(parsed);\n\
            - benches/measurement.rs:25:17: Measured routine returns Map<RepeatN<u64>, fn count_ones(u64) -> u32>, a must-use value that criterion drops\n            \
            bencher.iter(|| iter::repeat_n(1_u64, 64).map(u64::count_ones));\n\
            - benches/measurement.rs:28:17: Measured routine returns impl Fn() -> u64, a must-use value that criterion drops\n            \
            bencher.iter(|| || 64_u64);\n\
            - benches/measurement.rs:31:18: Measured routine returns Result<u64, ParseIntError>, a must-use value that criterion drops\n            \
            Bencher::iter(bencher, parsed);\n\
            - benches/measurement.rs:34:17: Measured routine returns Box<Result<u64, ParseIntError>, Global>, a must-use value that criterion drops\n            \
            bencher.iter_batched(|| \"64\", |input| Box::new(input.parse::<u64>()), BatchSize::SmallInput);\n\
            - benches/measurement.rs:37:17: Measured routine returns (u64, Result<u64, ParseIntError>), a must-use value that criterion drops\n            \
            bencher.iter(|| (64_u64, parsed()));\n\
            - benches/measurement.rs:40:17: Measured routine returns [Result<u64, ParseIntError>; 1], a must-use value that criterion drops\n            \
            bencher.iter(|| [parsed()]);\n\
            - benches/measurement.rs:43:17: Measured routine returns Box<dyn Iterator<Item = u64> + 'static, Global>, a must-use value that criterion drops\n            \
            bencher.iter(|| -> Box<dyn Iterator<Item = u64>> { Box::new(iter::empty()) });\n\
            - benches/measurement.rs:46:17: Measured routine returns impl Iterator<Item = u64>, a must-use value that criterion drops\n            \
            bencher.iter(numbers);\n\
            - benches/measurement.rs:49:45: Measured routine returns Result<u64, ParseIntError>, a must-use value that criterion drops\n            \
            bencher.to_async(ImmediateExecutor).iter(|| async { parsed() });\n\
            \n\
            ------------------------------\n\
            10 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
