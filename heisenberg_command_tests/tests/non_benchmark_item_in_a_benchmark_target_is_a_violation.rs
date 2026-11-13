use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn non_benchmark_item_in_a_benchmark_target_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "non_benchmark_item_in_benchmark_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NonBenchmarkItemInBenchmarkTarget: 1 violation\n\
            A helper used beyond its own benchmark file is a library, and libraries never live in a benches directory. Move it into a library crate, a test crate when only tests and benchmarks use it; when that crate depends on the package the benchmarks measure, move the benchmarks that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - benches/measurement.rs:12:14: Benchmark target holds an item that is neither its entry point, a constant, an import nor a private helper used only in its own file\n    \
            pub const fn sample_size() -> u64 {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
