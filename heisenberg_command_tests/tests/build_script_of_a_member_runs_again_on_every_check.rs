use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_SELECTION;
use heisenberg_command_tests::constants::JOBS_ARGUMENT;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn build_script_of_a_member_runs_again_on_every_check() {
    let workspace = FixtureWorkspace {
        name: "build_script_with_an_undeclared_input",
    }
    .path();
    let statuses = ["2", "3", "2"].map(|jobs| {
        HeisenbergCheck {
            workspace: &workspace,
        }
        .run_with_arguments(&[HOST_SELECTION.as_slice(), &[JOBS_ARGUMENT, jobs]].concat())
        .expect("the arguments must parse")
        .status
    });

    assert_eq!(
        statuses,
        [
            HeisenbergExitStatus::Compliant,
            HeisenbergExitStatus::Violations,
            HeisenbergExitStatus::Compliant
        ]
    );
}
