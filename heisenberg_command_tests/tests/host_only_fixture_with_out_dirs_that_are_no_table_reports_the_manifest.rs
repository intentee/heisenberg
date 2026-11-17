use std::path::Path;

use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn host_only_fixture_with_out_dirs_that_are_no_table_reports_the_manifest() {
    let reported = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("host_only_fixture_with_out_dirs_that_are_no_table")
        .join("Cargo.toml");

    assert_eq!(
        HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "host_only_fixture_with_out_dirs_that_are_no_table",
        }
        .path()
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "the out_dirs of the fixture manifest `{}` are not a table",
            reported.display()
        ))
    );
}
