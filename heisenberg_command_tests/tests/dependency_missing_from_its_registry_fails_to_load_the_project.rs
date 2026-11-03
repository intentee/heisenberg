use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn dependency_missing_from_its_registry_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "dependency_missing_from_its_registry",
    }
    .path();
    let crate_archive = workspace.join("registry").join("vendored-0.1.0.crate");
    let open_error = fs::read(&crate_archive).expect_err("the fixture registry has no archive");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the dependency sources of `{}` cannot be loaded\n\
             error: failed to download `vendored v0.1.0`\n\
             error: unable to get packages from source\n\
             error: failed to download replaced source registry `crates-io`\n\
             error: failed to open file `{}`\n\
             error: {open_error}\n",
            workspace.join("Cargo.toml").display(),
            crate_archive.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
