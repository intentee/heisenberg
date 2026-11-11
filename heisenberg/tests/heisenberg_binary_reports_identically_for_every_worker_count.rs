use std::env;
use std::path::Path;

use heisenberg_command_tests::binary_check::BinaryCheck;
use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::worker_count_outputs::WorkerCountOutputs;

#[test]
fn heisenberg_binary_reports_identically_for_every_worker_count() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let runs = [
        "binary_sharing_a_module_file_with_its_library",
        "unparsable_expansion_through_a_macro_chain",
    ]
    .map(|name| {
        BinaryCheck {
            binary: Path::new(env!("CARGO_BIN_EXE_heisenberg")),
            cargo_home: Path::new(&cargo_home),
            subcommand: CHECK_SUBCOMMAND,
            sysroot: Path::new(&sysroot),
            workspace: &FixtureWorkspace { name }.path(),
        }
        .spawn_for_both_worker_counts(&[])
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
