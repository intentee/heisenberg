use std::fs;
use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn coverage_export_in_another_format_fails_the_coverage_check() {
    let export = Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_another_format.json");

    fs::write(
        &export,
        r#"{"type":"llvm.coverage.json.export","version":"2.0.1"}"#,
    )
    .expect("the export must be written");

    let outcome = HeisenbergCoverageCheck {
        export: &export,
        workspace: &FixtureWorkspace {
            name: "coverage_fully_covered",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the coverage export cannot be used\n\
             error: the coverage export `{}` is of type `llvm.coverage.json.export` in version 2.0.1; Heisenberg reads `llvm.coverage.json.export` in version 3.1.0\n",
            export.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
