use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn member_outside_the_workspace_is_rejected() {
    let fixture = FixtureWorkspace {
        name: "member_outside_the_workspace_directory",
    }
    .path();
    let workspace = fixture.join("workspace");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: workspace member `{}` lies outside the workspace root `{}`\n",
            fixture.join("member").join("Cargo.toml").display(),
            workspace.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
