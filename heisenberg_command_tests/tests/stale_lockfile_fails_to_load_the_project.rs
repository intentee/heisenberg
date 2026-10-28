use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn stale_lockfile_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "stale_lockfile",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the dependencies of `{}` cannot be resolved from its lockfile\n\
             error: cannot update the lock file {} because --frozen was passed to prevent this\n\
             help: to generate the lock file without accessing the network, remove the --frozen flag and use --offline instead.\n",
            workspace.join("Cargo.toml").display(),
            workspace.join("Cargo.lock").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
