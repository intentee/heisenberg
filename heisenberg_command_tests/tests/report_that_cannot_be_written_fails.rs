use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn report_that_cannot_be_written_fails() {
    let workspace = FixtureWorkspace {
        name: "sibling_modules_importing_each_other",
    }
    .path();
    let mut read_only_output =
        File::open(workspace.join("Cargo.toml")).expect("the fixture manifest must open");
    let write_error = read_only_output
        .write_all(b"report")
        .expect_err("a file opened for reading must reject writes");
    let mut errors = Vec::new();
    let status = HeisenbergCheck {
        workspace: &workspace,
    }
    .toolchain_command()
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut read_only_output, &mut errors);

    assert_eq!(
        str::from_utf8(&errors),
        Ok(format!("error: the report cannot be written\nerror: {write_error}\n").as_str())
    );
    assert_eq!(status, HeisenbergExitStatus::Failure);
}
