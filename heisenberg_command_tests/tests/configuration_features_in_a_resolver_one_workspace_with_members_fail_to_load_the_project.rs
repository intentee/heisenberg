use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn configuration_features_in_a_resolver_one_workspace_with_members_fail_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "resolver_one_workspace_with_members",
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
             error: `{}` uses resolver 1 with a root package and other members, so cargo would apply the feature selection of the configuration `x86_64-pc-windows-msvc` to the root package only\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
