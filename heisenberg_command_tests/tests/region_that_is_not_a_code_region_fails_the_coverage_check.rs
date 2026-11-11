use std::path::Path;
use std::str;

use serde_json::Value;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::FIRST_REGION_KIND_POINTER;
use heisenberg_command_tests::constants::SKIPPED_REGION_KIND;
use heisenberg_command_tests::first_function_name::first_function_name;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tamper_export_value::tamper_export_value;
use heisenberg_command_tests::tamper_first_region_span::tamper_first_region_span;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;
use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_coverage_export::region_span::RegionSpan;

#[test]
fn region_that_is_not_a_code_region_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_fully_covered",
    };
    let TamperedExport {
        observed: function,
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_region_kind.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        tamper_first_region_span(
            export,
            RegionSpan {
                start: RegionPosition { line: 1, column: 1 },
                end: RegionPosition { line: 1, column: 1 },
            },
        )
        .expect("the first function has a region");
        tamper_export_value(
            export,
            FIRST_REGION_KIND_POINTER,
            Value::from(SKIPPED_REGION_KIND),
        )
        .expect("the tampered value must exist");
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
             error: the region at 1:1 of the function `{function}` in the coverage export `{export}` is of kind {SKIPPED_REGION_KIND}, which is not a code region\n",
            export = export.display(),
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
