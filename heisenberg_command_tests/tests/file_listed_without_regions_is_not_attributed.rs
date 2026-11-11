use std::path::Path;
use std::str;

use serde_json::Value;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::EXPORTED_DATA_POINTER;
use heisenberg_command_tests::constants::EXPORTED_FUNCTIONS_KEY;
use heisenberg_command_tests::constants::FIRST_FUNCTION_FILE_POINTER;
use heisenberg_command_tests::constants::FIRST_FUNCTION_FILENAMES_POINTER;
use heisenberg_command_tests::constants::FUNCTION_FIRST_FILE_POINTER;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;

#[test]
fn file_listed_without_regions_is_not_attributed() {
    let fixture = FixtureWorkspace {
        name: "coverage_inactive_module_file",
    };
    let workspace = fixture.path();
    let unlisted = Value::from(
        workspace
            .join("src")
            .join("coverage_only.rs")
            .to_str()
            .expect("the fixture path must be UTF-8"),
    );
    let TamperedExport {
        observed: (),
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_file_without_regions.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        export
            .pointer_mut(EXPORTED_DATA_POINTER)
            .and_then(Value::as_object_mut)
            .and_then(|data| data.get_mut(EXPORTED_FUNCTIONS_KEY))
            .and_then(Value::as_array_mut)
            .expect("the export holds its functions")
            .retain(|function| function.pointer(FUNCTION_FIRST_FILE_POINTER) != Some(&unlisted));
        export
            .pointer_mut(FIRST_FUNCTION_FILENAMES_POINTER)
            .and_then(Value::as_array_mut)
            .expect("the first function lists its files")
            .push(unlisted.clone());
        assert_ne!(export.pointer(FIRST_FUNCTION_FILE_POINTER), Some(&unlisted));
    })
    .expect("the export must be tampered with");
    let outcome = HeisenbergCoverageCheck {
        export: &export,
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n",
            coverage_footer(1, 1, 3).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
