use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn coverage_export_of_another_workspace_fails_the_coverage_check() {
    let exported = FixtureWorkspace {
        name: "coverage_uncovered_code",
    };
    let export = exported.coverage_export(COVERAGE_EXPORT_FILE);
    let workspace = FixtureWorkspace {
        name: "coverage_fully_covered",
    }
    .path();
    let outcome = HeisenbergCoverageCheck {
        export: &export,
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the coverage export cannot be used\n\
             error: the coverage export `{}` covers the workspace of `{}`, not the workspace of `{}`\n",
            export.display(),
            exported.path().join("Cargo.toml").display(),
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
