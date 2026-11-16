use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn out_dir_pinned_to_a_version_the_workspace_does_not_resolve_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "out_dir_of_an_unresolved_version",
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
             error: `{}` declares an out-dir for `bindings_sys` 0.2.0, and the workspace resolves `bindings_sys` 0.1.0: declare the build script output of the resolved version\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
