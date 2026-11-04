use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn heisenberg_settings_with_an_unknown_key_fail_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "heisenberg_settings_with_an_unknown_key",
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
             error: the [workspace.metadata.heisenberg] settings of `{}` are invalid\n\
             error: unknown field `ignore`, expected one of `out_dirs`, `targets`, `test_crates`\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
