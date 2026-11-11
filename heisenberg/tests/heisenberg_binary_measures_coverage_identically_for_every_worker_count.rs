use std::env;
use std::path::Path;

use heisenberg_command_tests::binary_check::BinaryCheck;
use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::COVERAGE_CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_ARGUMENT;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::worker_count_outputs::WorkerCountOutputs;

#[test]
fn heisenberg_binary_measures_coverage_identically_for_every_worker_count() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let runs = ["coverage_path_module", "coverage_uncovered_code"].map(|name| {
        let fixture = FixtureWorkspace { name };

        BinaryCheck {
            binary: Path::new(env!("CARGO_BIN_EXE_heisenberg")),
            cargo_home: Path::new(&cargo_home),
            subcommand: COVERAGE_CHECK_SUBCOMMAND,
            sysroot: Path::new(&sysroot),
            workspace: &fixture.path(),
        }
        .spawn_for_both_worker_counts(&[
            COVERAGE_EXPORT_ARGUMENT,
            fixture
                .coverage_export(COVERAGE_EXPORT_FILE)
                .to_str()
                .expect("the fixture export path is UTF-8"),
        ])
        .expect("the heisenberg binary must start")
    });

    for run in runs {
        let WorkerCountOutputs {
            default_workers,
            single_worker,
        } = run.outputs().expect("the heisenberg binary must finish");

        assert_eq!(default_workers.status.code(), Some(1));
        assert_eq!(single_worker, default_workers);
    }
}
