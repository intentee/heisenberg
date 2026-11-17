use std::path::Path;

use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn host_only_fixture_with_a_configuration_name_that_is_no_string_reports_the_configuration() {
    let reported = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("host_only_fixture_with_a_configuration_name_that_is_no_string")
        .join("Cargo.toml");

    assert_eq!(
        HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "host_only_fixture_with_a_configuration_name_that_is_no_string",
        }
        .path()
        .err()
        .map(|error| error.to_string()),
        Some(format!(
            "the name of configuration 0 of the fixture manifest `{}` is not a string",
            reported.display()
        ))
    );
}
