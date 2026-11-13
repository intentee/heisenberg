use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn error_that_cannot_be_written_is_returned_to_the_process() {
    let workspace = FixtureWorkspace {
        name: "sibling_modules_importing_each_other",
    }
    .path();
    let mut read_only_output =
        File::open(workspace.join("Cargo.toml")).expect("the fixture manifest must open");
    let mut read_only_errors =
        File::open(workspace.join("Cargo.toml")).expect("the fixture manifest must open");
    let write_error = File::open(workspace.join("Cargo.toml"))
        .expect("the fixture manifest must open")
        .write_all(b"error")
        .expect_err("a file opened for reading must reject writes");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .toolchain_command()
    .expect("the arguments must parse")
    .run(
        &BTreeMap::new(),
        &mut read_only_output,
        &mut read_only_errors,
    );

    assert_eq!(
        outcome.err().map(|error| error.to_string()),
        Some(write_error.to_string())
    );
}
