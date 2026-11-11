use std::fs;
use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn coverage_export_that_is_not_json_fails_the_coverage_check() {
    let export = Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_not_json.json");

    fs::write(&export, "{").expect("the export must be written");

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
             error: the coverage export `{}` is not valid JSON\n\
             error: EOF while parsing an object at line 1 column 1\n",
            export.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
