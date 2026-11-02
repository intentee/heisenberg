use std::fs;
use std::path::Path;
use std::str;

use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_binary::HeisenbergBinary;

#[test]
fn heisenberg_binary_runs_without_spawning_processes() {
    let fixture = FixtureWorkspace {
        name: "process_free_check",
    }
    .path();
    let cargo_home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cargo_home");

    fs::create_dir_all(&cargo_home).expect("the cargo home must be created");

    let output = HeisenbergBinary {
        cargo_home: &cargo_home,
        executable: Path::new(env!("CARGO_BIN_EXE_heisenberg")),
        sysroot: &fixture.join("sysroot"),
        workspace: &fixture.join("workspace"),
    }
    .run()
    .expect("the heisenberg binary must start");

    assert_eq!(str::from_utf8(&output.stderr), Ok(""));
    assert_eq!(
        str::from_utf8(&output.stdout),
        Ok("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(output.status.code(), Some(0));
}
