use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn regions_in_a_file_outside_every_module_tree_fail_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_inactive_module_file",
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
             error: the coverage export `{}` records regions in `{}`, a file of a workspace member that no crate target of the analysed configuration includes as a module file; export the coverage of a run of the analysed configuration\n",
            export.display(),
            workspace.join("src").join("coverage_only.rs").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
