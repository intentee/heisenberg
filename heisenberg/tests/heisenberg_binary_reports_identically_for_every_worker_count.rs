use std::env;
use std::path::Path;
use std::process::Child;

use heisenberg_command_tests::binary_check::BinaryCheck;
use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;

#[test]
fn heisenberg_binary_reports_identically_for_every_worker_count() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let runs: Vec<[Child; 2]> = [
        "binary_sharing_a_module_file_with_its_library",
        "unparsable_expansion_through_a_macro_chain",
    ]
    .map(|name| {
        let workspace = FixtureWorkspace { name }.path();
        let check = BinaryCheck {
            binary: Path::new(env!("CARGO_BIN_EXE_heisenberg")),
            cargo_home: Path::new(&cargo_home),
            sysroot: Path::new(&sysroot),
            workspace: &workspace,
        };

        [["--jobs", "1"].as_slice(), &[]].map(|arguments| {
            check
                .spawn(arguments)
                .expect("the heisenberg binary must start")
        })
    })
    .into_iter()
    .collect();

    for [single_worker, default_workers] in runs {
        let single_worker = single_worker
            .wait_with_output()
            .expect("the heisenberg binary must finish");
        let default_workers = default_workers
            .wait_with_output()
            .expect("the heisenberg binary must finish");

        assert_eq!(default_workers.status.code(), Some(1));
        assert_eq!(single_worker, default_workers);
    }
}
