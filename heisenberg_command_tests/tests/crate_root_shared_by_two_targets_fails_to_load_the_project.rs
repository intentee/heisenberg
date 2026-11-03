use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn crate_root_shared_by_two_targets_fails_to_load_the_project() {
    let fixture = FixtureWorkspace {
        name: "crate_root_shared_by_two_targets",
    }
    .path();
    let workspace = fixture.clone();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: crate root `{}` belongs to both crate_root_shared_by_two_targets lib and crate_root_shared_by_two_targets bin \"crate_root_shared_by_two_targets\"\n",
            fixture.join("src").join("main.rs").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
