use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn region_in_code_the_configuration_leaves_inactive_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_inactive_region",
    };
    let export = fixture.coverage_export(COVERAGE_EXPORT_FILE);
    let workspace = fixture.path();
    let outcome = HeisenbergCoverageCheck {
        export: &export,
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the coverage of the configuration `{HOST_CONFIGURATION}` cannot be measured\n\
             error: the coverage export `{}` records a region at 6:1 of `{}`, in code that the analysed configuration leaves inactive; export the coverage of a run of the analysed configuration, and make production code independent of cfg(coverage), which cargo llvm-cov sets unless it runs with --no-cfg-coverage\n",
            export.display(),
            workspace.join("src").join("lib.rs").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
