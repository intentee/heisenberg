use std::env;
use std::path::Path;
use std::str;

use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::constants::CARGO_HOME_VARIABLE;
use heisenberg_test_harness::constants::SYSROOT_VARIABLE;
use heisenberg_test_harness::heisenberg_binary::HeisenbergBinary;

#[test]
fn heisenberg_binary_analyses_the_real_sysroot_without_spawning_processes() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let output = HeisenbergBinary {
        cargo_home: Path::new(&cargo_home),
        executable: Path::new(env!("CARGO_BIN_EXE_heisenberg")),
        sysroot: Path::new(&sysroot),
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run()
    .expect("the heisenberg binary must start");

    assert_eq!(str::from_utf8(&output.stderr), Ok(""));
    assert_eq!(
        str::from_utf8(&output.stdout),
        Ok("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(output.status.code(), Some(0));
}
