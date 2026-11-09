use std::fs;
use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn source_file_that_is_not_utf8_fails_to_load_the_project() {
    let workspace = HostOnlyFixture {
        directory: &Path::new(env!("CARGO_TARGET_TMPDIR")).join("source_file_that_is_not_utf8"),
        name: "functions_used_only_by_tests",
    }
    .path()
    .expect("the host-only fixture must be prepared");
    let fixture_directory = workspace
        .join("library")
        .join("src")
        .join("fixtures")
        .join("non_utf8_source");
    let not_utf8 = fixture_directory.join("lib.rs");

    fs::create_dir_all(&fixture_directory)
        .and_then(|()| fs::write(&not_utf8, [0xFF]))
        .expect("the non-UTF-8 source file must be written");

    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded for the configuration `{HOST_CONFIGURATION}`\n\
             error: `{}` is not UTF-8, so rust-analyzer loads it without text\n\
             error: invalid utf-8 sequence of 1 bytes from index 0\n",
            not_utf8.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
