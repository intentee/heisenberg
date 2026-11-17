use std::path::Path;

use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn host_only_fixture_of_a_missing_workspace_reports_what_it_cannot_copy() {
    let reported = FixtureWorkspace {
        name: "host_only_fixture_that_does_not_exist",
    }
    .path();

    assert_eq!(
        HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "host_only_fixture_that_does_not_exist",
        }
        .path()
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "`{}` cannot be copied into the host-only fixture",
            reported.display()
        ))
    );
}
