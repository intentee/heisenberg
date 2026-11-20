use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn attribute_macro_that_rust_analyzer_cannot_expand_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_failing_attribute",
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
             error: the macro invocation at src/lib.rs:1:3 fails to expand under rust-analyzer, so the functions it writes cannot be measured: proc-macro panicked: rust-analyzer expands this attribute\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
