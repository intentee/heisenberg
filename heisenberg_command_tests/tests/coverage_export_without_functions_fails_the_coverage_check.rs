use std::path::Path;
use std::str;

use serde_json::Value;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::EXPORTED_DATA_POINTER;
use heisenberg_command_tests::constants::EXPORTED_FUNCTIONS_KEY;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;

#[test]
fn coverage_export_without_functions_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_fully_covered",
    };
    let TamperedExport {
        observed: (),
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_without_functions.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        export
            .pointer_mut(EXPORTED_DATA_POINTER)
            .and_then(Value::as_object_mut)
            .expect("the export holds its data")
            .remove(EXPORTED_FUNCTIONS_KEY);
    })
    .expect("the export must be tampered with");
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
             error: the coverage export `{export}` does not hold the functions and regions of a `cargo llvm-cov report --json` export made without --summary-only and --skip-functions\n\
             error: missing field `functions`\n",
            export = export.display(),
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
