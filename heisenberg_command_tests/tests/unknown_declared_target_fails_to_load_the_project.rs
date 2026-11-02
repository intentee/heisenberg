use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn unknown_declared_target_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "unknown_declared_target",
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
             error: `aarch64-unknown-linux-gnu` in `{}` is not a supported target; the supported targets are aarch64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu\n",
            workspace.join("Cargo.toml").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
