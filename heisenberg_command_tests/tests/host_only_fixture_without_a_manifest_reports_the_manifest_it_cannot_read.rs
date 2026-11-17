use std::path::Path;

use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn host_only_fixture_without_a_manifest_reports_the_manifest_it_cannot_read() {
    let reported = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("host_only_fixture_without_a_manifest")
        .join("Cargo.toml");

    assert_eq!(
        HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "host_only_fixture_without_a_manifest",
        }
        .path()
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "the fixture manifest `{}` cannot be read",
            reported.display()
        ))
    );
}
