use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn benchmarks_package_without_libraries_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "benchmarks_package_without_libraries",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["BenchmarksOnlyPackageWithoutLibraries: 1 violation\n\
            Benchmarks belong to the package whose code they measure, in its benches directory, and a benchmark that needs a library which depends on that package, such as a test harness, belongs beside that library in its package. Heisenberg cannot tell from the code which package the benchmarks measure, so it lists every workspace member they use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they measure.\n\
            \n\
            - library_benchmarks/Cargo.toml: Package library_benchmarks holds only benchmarks and no library of its own, so the package its benchmarks measure must own them\n      \
            cannot move to base: base v0.1.0 -> library v0.1.0 -> base v0.1.0\n      \
            could move without a dependency cycle to library\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(3, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
