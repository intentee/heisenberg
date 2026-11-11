use std::fs::File;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn coverage_export_that_cannot_be_opened_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_fully_covered",
    };
    let export = fixture.coverage_export("missing-llvm-cov.json");
    let open_error = File::open(&export).expect_err("the fixture holds no such export");
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
             error: the coverage export `{}` cannot be opened\n\
             error: {open_error}\n",
            export.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
