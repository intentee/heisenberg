use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn feature_selected_with_a_package_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "feature_gated_cycle",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run_with_arguments(&["--features", "feature_gated_cycle/cyclic"])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `feature_gated_cycle/cyclic` is not the name of a feature; --features takes only the names of features that workspace members define\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
