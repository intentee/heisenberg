use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn out_dir_inside_the_target_directory_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "out_dir_inside_the_target_directory",
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
             error: `{}` declares the out-dir `target/generated` of `bindings_sys` for the configuration `x86_64-pc-windows-msvc` inside the target directory, which holds what earlier builds left behind: commit the declared build script output outside it\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
