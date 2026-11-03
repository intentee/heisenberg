use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use std::str;

use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::LLVM_PROFILE_FILE_VARIABLE;
use heisenberg_command_tests::constants::PATH_VARIABLE;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;

fn heisenberg_check(cargo_home: &Path, sysroot: &Path, workspace: &Path) -> io::Result<Output> {
    Command::new(env!("CARGO_BIN_EXE_heisenberg"))
        .env_clear()
        .env(PATH_VARIABLE, "")
        .envs(env::vars_os().filter(|(name, _)| name == LLVM_PROFILE_FILE_VARIABLE))
        .arg("check")
        .arg("--cargo-home")
        .arg(cargo_home)
        .arg("--sysroot")
        .arg(sysroot)
        .arg("--workspace")
        .arg(workspace)
        .output()
}

#[test]
fn heisenberg_binary_analyses_the_real_sysroot_without_spawning_processes() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let output = heisenberg_check(
        Path::new(&cargo_home),
        Path::new(&sysroot),
        &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    )
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

#[test]
fn heisenberg_binary_runs_without_spawning_processes() {
    let fixture = FixtureWorkspace {
        name: "process_free_check",
    }
    .path();
    let cargo_home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cargo_home");

    fs::create_dir_all(&cargo_home).expect("the cargo home must be created");

    let output = heisenberg_check(
        &cargo_home,
        &fixture.join("sysroot"),
        &fixture.join("workspace"),
    )
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
