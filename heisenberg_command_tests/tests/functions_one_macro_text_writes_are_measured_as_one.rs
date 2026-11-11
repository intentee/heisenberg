use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn functions_one_macro_text_writes_are_measured_as_one() {
    let fixture = FixtureWorkspace {
        name: "coverage_macro_text_shared_by_functions",
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
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n",
            coverage_footer(1, 1, 5).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
