use std::fs;
use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn json_without_an_export_type_fails_the_coverage_check() {
    let export = Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_without_export_type.json");

    fs::write(&export, "{}").expect("the export must be written");

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
             error: the coverage export `{}` does not name the type and version of an llvm-cov JSON export\n\
             error: missing field `type`\n",
            export.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
