use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str;

#[test]
fn heisenberg_binary_runs_without_spawning_processes() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");
    let cargo_home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cargo_home");

    fs::create_dir_all(&cargo_home).expect("the cargo home must be created");

    let mut command = Command::new(env!("CARGO_BIN_EXE_heisenberg"));

    command.env_clear().env("PATH", "");

    if let Some(profile_file) = env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile_file);
    }

    let output = command
        .arg("check")
        .arg("--cargo-home")
        .arg(&cargo_home)
        .arg("--sysroot")
        .arg(fixtures.join("sysroot"))
        .arg("--workspace")
        .arg(fixtures.join("workspace"))
        .output()
        .expect("the heisenberg binary must start");

    assert_eq!(str::from_utf8(&output.stderr), Ok(""));
    assert_eq!(
        str::from_utf8(&output.stdout),
        Ok("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(output.status.code(), Some(0));
}
