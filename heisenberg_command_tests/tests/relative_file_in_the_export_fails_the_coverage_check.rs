use std::path::Path;
use std::str;

use serde_json::Value;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::FIRST_FUNCTION_FILE_POINTER;
use heisenberg_command_tests::first_function_name::first_function_name;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tamper_export_value::tamper_export_value;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;

#[test]
fn relative_file_in_the_export_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_fully_covered",
    };
    let TamperedExport {
        observed: function,
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_relative_file.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        tamper_export_value(
            export,
            FIRST_FUNCTION_FILE_POINTER,
            Value::from("src/lib.rs"),
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
             error: the function `{function}` of the coverage export `{export}` names a relative file; export the coverage of a run without --remap-path-prefix\n\
             error: `src/lib.rs` is not an absolute path\n",
            export = export.display(),
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
