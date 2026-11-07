use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn configuration_with_an_unknown_target_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "configuration_with_an_unknown_target",
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
             error: the configuration `aarch64-unknown-linux-gnu` of `{}` targets `aarch64-unknown-linux-gnu`, which is not a supported target; the supported targets are aarch64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
