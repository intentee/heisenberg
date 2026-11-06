use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn unknown_selected_feature_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "feature_gated_cycle",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run_with_arguments(&["--features", "missing"])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: feature `missing` is selected with --features, but no member of `{}` defines it\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
