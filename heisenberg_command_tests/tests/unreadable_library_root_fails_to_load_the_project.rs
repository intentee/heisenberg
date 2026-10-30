use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn unreadable_library_root_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "library_root_unreadable",
    }
    .path();
    let library_root = workspace.join("src").join("missing.rs");
    let read_error = fs::read(&library_root).expect_err("the fixture has no library root");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the library root `{}` cannot be read\n\
             error: {read_error}\n",
            library_root.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
