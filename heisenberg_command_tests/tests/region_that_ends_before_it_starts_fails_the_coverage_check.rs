use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::first_function_name::first_function_name;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tamper_first_region_span::tamper_first_region_span;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;
use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_coverage_export::region_span::RegionSpan;

#[test]
fn region_that_ends_before_it_starts_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_fully_covered",
    };
    let TamperedExport {
        observed: function,
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("coverage_region_ending_before_its_start.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        tamper_first_region_span(
            export,
            RegionSpan {
                start: RegionPosition { line: 2, column: 1 },
                end: RegionPosition { line: 1, column: 1 },
            },
        )
        .expect("the first function has a region");
        first_function_name(export)
    })
    .expect("the export must be tampered with");
    let function = function.expect("the export names its first function");
    let outcome = HeisenbergCoverageCheck {
        export: &export,
        workspace: &fixture.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the coverage export cannot be used\n\
             error: the region from 2:1 to 1:1 of the function `{function}` in the coverage export `{export}` ends before it starts; export the coverage with cargo llvm-cov\n",
            export = export.display(),
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
