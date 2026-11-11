use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn macro_call_that_rust_analyzer_cannot_resolve_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_macro_hidden_from_rust_analyzer",
    };
    let outcome = HeisenbergCoverageCheck {
        export: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
        workspace: &fixture.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the coverage of the configuration `{HOST_CONFIGURATION}` cannot be measured\n\
             error: rust-analyzer cannot resolve the macro of the call at src/lib.rs:11:1, so the functions it writes cannot be measured\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
