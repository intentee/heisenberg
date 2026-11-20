use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn uncovered_code_of_a_path_module_is_reported_where_it_is_written() {
    let fixture = FixtureWorkspace {
        name: "coverage_path_module",
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
            "UncoveredCode: 1 violation\n\
            Every line of production code runs under the tests. Add tests that execute the code, or remove the code that nothing needs.\n\
            \n\
            - shared/clamp.rs:1:8: No test executes parts of the function\n    \
            pub fn clamp(value: u32) -> u32 {\n      \
            not executed at shared/clamp.rs:2:22\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n",
            coverage_footer(1, 2, 8).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
