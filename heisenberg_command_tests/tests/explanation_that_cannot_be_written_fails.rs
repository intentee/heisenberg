use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;

#[test]
fn explanation_that_cannot_be_written_fails() {
    let mut read_only_output = File::open(
        FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path()
        .join("Cargo.toml"),
    )
    .expect("the fixture manifest must open");
    let write_error = read_only_output
        .write_all(b"explanation")
        .expect_err("a file opened for reading must reject writes");
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from(["heisenberg", "explain", "GlobImport"])
        .expect("the arguments must parse")
        .run(&BTreeMap::new(), &mut read_only_output, &mut errors);

    assert_eq!(
        str::from_utf8(&errors),
        Ok(format!("error: the explanation cannot be written\nerror: {write_error}\n").as_str())
    );
    assert_eq!(status, HeisenbergExitStatus::Failure);
}
