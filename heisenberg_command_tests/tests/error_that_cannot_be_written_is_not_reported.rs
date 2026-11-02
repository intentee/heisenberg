use std::collections::BTreeMap;
use std::fs::File;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn error_that_cannot_be_written_is_not_reported() {
    let workspace = FixtureWorkspace {
        name: "sibling_modules_importing_each_other",
    }
    .path();
    let mut read_only_output =
        File::open(workspace.join("Cargo.toml")).expect("the fixture manifest must open");
    let mut read_only_errors =
        File::open(workspace.join("Cargo.toml")).expect("the fixture manifest must open");
    let status = HeisenbergCheck {
        workspace: &workspace,
    }
    .toolchain_command()
    .expect("the arguments must parse")
    .run(
        &BTreeMap::new(),
        &mut read_only_output,
        &mut read_only_errors,
    );

    assert_eq!(status, HeisenbergExitStatus::ErrorNotReported);
}
