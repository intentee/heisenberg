use std::collections::BTreeMap;
use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn sysroot_without_rust_src_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "sibling_modules_importing_each_other",
    }
    .path();
    let library_manifest = workspace
        .join("lib")
        .join("rustlib")
        .join("src")
        .join("rust")
        .join("library")
        .join("Cargo.toml");
    let read_error = fs::read(&library_manifest).expect_err("the fixture has no rust-src");
    let outcome = CheckOutcome::of(
        &HeisenbergCheck {
            workspace: &workspace,
        }
        .command(&workspace)
        .expect("the arguments must parse"),
        &BTreeMap::new(),
    );

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the cargo workspace `{0}` cannot be loaded\n\
             error: failed to read `{0}`\n\
             error: {read_error}\n",
            library_manifest.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
