use std::path::Path;

use heisenberg_command_tests::constants::PACKAGE_DIRECTORY;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;

#[test]
fn tampered_export_of_a_source_that_is_no_json_reports_the_export_it_cannot_parse() {
    let source = Path::new(PACKAGE_DIRECTORY).join("Cargo.toml");
    let copy = Path::new(env!("CARGO_TARGET_TMPDIR")).join("unwritten_export.json");

    assert_eq!(
        TamperedCoverageExport {
            copy: &copy,
            source: &source,
        }
        .written(|_| ())
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "the coverage export `{}` cannot be parsed",
            source.display()
        ))
    );
}
