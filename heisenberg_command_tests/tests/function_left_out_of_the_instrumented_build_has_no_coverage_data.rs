use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn function_left_out_of_the_instrumented_build_has_no_coverage_data() {
    let fixture = FixtureWorkspace {
        name: "coverage_without_coverage_data",
    };
    let outcome = HeisenbergCoverageCheck {
        export: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
        workspace: &fixture.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "CodeWithoutCoverageData: 2 violations\n\
            Every production function is measured. Build and run the target that holds the function under the instrumented tests, and keep the function instrumented: rustc records no coverage for #[automatically_derived] code, and cfg(coverage) leaves code out of the instrumented build.\n\
            \n\
            - src/lib.rs:5:8: The coverage export holds no data for the function\n        \
            fn clone(&self) -> Self {\n\
            - src/lib.rs:11:8: The coverage export holds no data for the function\n    \
            pub fn uninstrumented() -> u32 {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n",
            coverage_footer(1, 3, 3).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
