use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn unexecuted_production_code_is_uncovered_code() {
    let fixture = FixtureWorkspace {
        name: "coverage_uncovered_code",
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
            "UncoveredCode: 4 violations\n\
            Every line of production code runs under the tests. Add tests that execute the code, or remove the code that nothing needs.\n\
            \n\
            - src/lib.rs:1:8: No test executes parts of the function\n    \
            pub fn sign(value: i32) -> &'static str {\n      \
            not executed at src/lib.rs:2:20\n\
            - src/lib.rs:5:8: No test executes the function\n    \
            pub fn never_called() -> u32 {\n\
            - src/lib.rs:9:8: No test executes parts of the function\n    \
            pub fn label(name: &str) -> String {\n      \
            not executed at src/lib.rs:10:25\n      \
            not executed at src/lib.rs:10:34\n\
            - src/lib.rs:13:8: No test executes parts of the function\n    \
            pub fn incremented(values: &[u32]) -> Vec<u32> {\n      \
            not executed at src/lib.rs:14:31\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n",
            coverage_footer(1, 4, 22).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
