use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_SELECTION;
use heisenberg_command_tests::constants::JOBS_ARGUMENT;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn file_left_in_the_out_dir_by_an_earlier_run_is_not_analysed() {
    let workspace = FixtureWorkspace {
        name: "build_script_leaving_a_file_behind",
    }
    .path();
    let statuses = ["3", "2"].map(|jobs| {
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
            HeisenbergExitStatus::Violations
        ]
    );
}
