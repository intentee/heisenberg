use std::path::Path;

use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::PACKAGE_DIRECTORY;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;

#[test]
fn tampered_export_that_cannot_be_written_reports_its_copy() {
    let source = FixtureWorkspace {
        name: "coverage_fully_covered",
    }
    .coverage_export(COVERAGE_EXPORT_FILE);
    let copy = Path::new(PACKAGE_DIRECTORY)
        .join("Cargo.toml")
        .join("below.json");

    assert_eq!(
        TamperedCoverageExport {
            copy: &copy,
            source: &source,
        }
        .written(|_| ())
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "the tampered coverage export `{}` cannot be written",
            copy.display()
        ))
    );
}
