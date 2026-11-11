use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_SELECTION;
use heisenberg_command_tests::constants::JOBS_ARGUMENT;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn worker_count_holds_for_the_whole_process() {
    let workspace = FixtureWorkspace {
        name: "acyclic_modules",
    }
    .path();
    let check = HeisenbergCheck {
        workspace: &workspace,
    };
    let single_worker = check
        .run_with_arguments(&[HOST_SELECTION.as_slice(), &[JOBS_ARGUMENT, "1"]].concat())
        .expect("the arguments must parse");
    let default_workers = check.run().expect("the arguments must parse");

    assert_eq!(single_worker.status, HeisenbergExitStatus::Compliant);
    assert_eq!(
        str::from_utf8(&default_workers.errors),
        Ok("error: the analysis cannot run on the worker pool\n\
            error: the worker pool of this process runs 1 workers, so it cannot run 8\n")
    );
    assert_eq!(default_workers.status, HeisenbergExitStatus::Failure);
}
