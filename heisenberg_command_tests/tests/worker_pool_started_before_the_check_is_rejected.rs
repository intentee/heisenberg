use std::str;

use rayon::ThreadPoolBuilder;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn worker_pool_started_before_the_check_is_rejected() {
    ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .expect("no worker pool runs before the test");

    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the analysis cannot run on the worker pool\n\
            error: the worker pool of 8 workers cannot be started\n\
            error: The global thread pool has already been initialized.\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
