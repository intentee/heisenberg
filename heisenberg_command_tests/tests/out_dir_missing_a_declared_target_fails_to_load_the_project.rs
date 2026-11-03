use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn out_dir_missing_a_declared_target_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "out_dir_missing_a_declared_target",
    }
    .path();
    let target_directory = workspace
        .join("bindings_sys")
        .join("generated")
        .join("x86_64-pc-windows-msvc");
    let read_error =
        fs::read_dir(&target_directory).expect_err("the fixture has no windows out-dir");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the declared out-dir `{}` cannot be read\n\
             error: {read_error}\n",
            target_directory.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
