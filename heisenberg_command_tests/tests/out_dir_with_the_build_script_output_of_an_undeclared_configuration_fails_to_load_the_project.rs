use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn out_dir_with_the_build_script_output_of_an_undeclared_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "out_dir_with_an_undeclared_configuration_output",
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
             error: `{}` declares an out-dir for `bindings_sys` with the build script output of `aarch64-apple-darwin`, which is not a declared configuration\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
