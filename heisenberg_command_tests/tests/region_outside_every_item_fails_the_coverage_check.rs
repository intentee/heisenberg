use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;
use heisenberg_command_tests::tamper_first_region_span_of_file::tamper_first_region_span_of_file;
use heisenberg_command_tests::tampered_coverage_export::TamperedCoverageExport;
use heisenberg_command_tests::tampered_export::TamperedExport;
use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_coverage_export::region_span::RegionSpan;

#[test]
fn region_outside_every_item_fails_the_coverage_check() {
    let fixture = FixtureWorkspace {
        name: "coverage_uncovered_code",
    };
    let workspace = fixture.path();
    let TamperedExport {
        observed: (),
        path: export,
    } = TamperedCoverageExport {
        copy: &Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("coverage_region_outside_every_item.json"),
        source: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
    }
    .written(|export| {
        tamper_first_region_span_of_file(
            export,
            &workspace.join("src").join("lib.rs"),
            RegionSpan {
                start: RegionPosition { line: 4, column: 1 },
                end: RegionPosition { line: 4, column: 1 },
            },
        )
        .expect("the first function of the library has a region");
    })
    .expect("the export must be tampered with");
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
             error: the coverage export `{}` records a region at {}:{} of `{}`, outside every item of the current text of the file; export the coverage of a run of the current sources\n",
            export.display(),
            4,
            1,
            workspace.join("src").join("lib.rs").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
