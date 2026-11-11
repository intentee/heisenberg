use std::fs;
use std::fs::File;
use std::path::Path;
use std::str;
use std::time::SystemTime;

use humantime::format_rfc3339_nanos;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn coverage_export_older_than_a_measured_file_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_uncovered_code",
    };
    let workspace = fixture.path();
    let library = workspace.join("src").join("lib.rs");
    let export = Path::new(env!("CARGO_TARGET_TMPDIR")).join("coverage_outdated.json");

    fs::copy(fixture.coverage_export(COVERAGE_EXPORT_FILE), &export)
        .expect("the export must be copied");
    File::options()
        .write(true)
        .open(&export)
        .and_then(|file| file.set_modified(SystemTime::UNIX_EPOCH))
        .expect("the copy must be dated before its sources");

    let library_modified = fs::metadata(&library)
        .and_then(|metadata| metadata.modified())
        .expect("the modification time of the library must be readable");
    let outcome = HeisenbergCoverageCheck {
        export: &export,
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the coverage of the configuration `{HOST_CONFIGURATION}` cannot be measured\n\
             error: the coverage export `{}`, modified at {}, is older than `{}`, modified at {}; export the coverage of a run of the current sources\n",
            export.display(),
            format_rfc3339_nanos(SystemTime::UNIX_EPOCH),
            library.display(),
            format_rfc3339_nanos(library_modified)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
