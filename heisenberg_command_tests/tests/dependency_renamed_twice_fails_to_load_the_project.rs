use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn dependency_renamed_twice_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "dependency_renamed_twice",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");
    let package = format!("dependency_renamed_twice v0.1.0 ({})", workspace.display());
    let dependency = format!("shared v0.1.0 ({})", workspace.join("shared").display());

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: `{package}` depends on `{dependency}` under more than one crate name\n\
             error: the crate `{package}` depends on crate `{dependency}` multiple times with different names\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
