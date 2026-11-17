use std::path::Path;

use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn host_only_fixture_with_configurations_that_are_no_tables_reports_the_manifest() {
    let reported = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("host_only_fixture_with_configurations_that_are_no_tables")
        .join("Cargo.toml");

    assert_eq!(
        HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "host_only_fixture_with_configurations_that_are_no_tables",
        }
        .path()
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "the configurations of the fixture manifest `{}` are not an array of tables",
            reported.display()
        ))
    );
}
