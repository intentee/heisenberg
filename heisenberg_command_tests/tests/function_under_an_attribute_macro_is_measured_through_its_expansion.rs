use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn function_under_an_attribute_macro_is_measured_through_its_expansion() {
    let fixture = FixtureWorkspace {
        name: "coverage_attribute_macro_item",
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
            - src/lib.rs:5:8: No test executes parts of the function\n    \
            pub fn describe(value: u32) -> String {\n      \
            not executed at src/lib.rs:6:55\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n",
            coverage_footer(1, 1, 5).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
