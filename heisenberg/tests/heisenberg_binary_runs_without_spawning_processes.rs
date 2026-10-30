use std::env;
use std::process::Command;
use std::str;

use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn heisenberg_binary_runs_without_spawning_processes() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_heisenberg"));

    command.env_clear().env("PATH", "");

    if let Some(profile_file) = env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile_file);
    }

    let output = command
        .arg("check")
        .arg("--cargo-home")
        .arg(env::var_os("HEISENBERG_CARGO_HOME").expect("HEISENBERG_CARGO_HOME must be exported"))
        .arg("--sysroot")
        .arg(env::var_os("HEISENBERG_SYSROOT").expect("HEISENBERG_SYSROOT must be exported"))
        .arg("--workspace")
        .arg(
            FixtureWorkspace {
                name: "acyclic_modules",
            }
            .path(),
        )
        .output()
        .expect("the heisenberg binary must start");

    assert_eq!(str::from_utf8(&output.stderr), Ok(""));
    assert_eq!(
        str::from_utf8(&output.stdout),
        Ok("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(output.status.code(), Some(0));
}
