use std::path::Path;
use std::str;

use serde_json::Value;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::FIRST_REGION_FILE_ID_POINTER;
use heisenberg_command_tests::first_function_name::first_function_name;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tamper_export_value::tamper_export_value;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;

#[test]
fn region_of_a_file_the_function_does_not_list_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_fully_covered",
    };
    let TamperedExport {
        observed: function,
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("coverage_region_file_out_of_range.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        tamper_export_value(export, FIRST_REGION_FILE_ID_POINTER, Value::from(1))
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
             error: a region of the function `{function}` in the coverage export `{export}` refers to file 1, past the end of its list of filenames, which holds 1\n",
            export = export.display(),
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
