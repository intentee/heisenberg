use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn feature_selected_with_dependency_syntax_is_rejected_by_cargo() {
    let workspace = FixtureWorkspace {
        name: "feature_gated_cycle",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run_with_arguments(&["--features", "dep:cyclic"])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: cargo rejects the features selected with --features\n\
            error: feature `dep:cyclic` is not allowed to use explicit `dep:` syntax\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
