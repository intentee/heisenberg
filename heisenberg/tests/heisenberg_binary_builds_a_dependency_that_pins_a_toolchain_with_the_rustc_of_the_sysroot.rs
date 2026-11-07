use std::env;
use std::process::Command;
use std::str;

use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::HOST_SELECTION;
use heisenberg_command_tests::constants::RUSTUP_TOOLCHAIN_VARIABLE;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::host_footer::host_footer;
use heisenberg_project_model::constants::CLIPPY_ARGUMENTS_VARIABLE;
use heisenberg_project_model::constants::CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE;
use heisenberg_project_model::constants::COMPILER_VARIABLES;
use heisenberg_project_model::constants::NIGHTLY_FEATURE_VARIABLES;
use heisenberg_project_model::constants::RUSTFLAGS_VARIABLES;

#[test]
fn heisenberg_binary_builds_a_dependency_that_pins_a_toolchain_with_the_rustc_of_the_sysroot() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let mut heisenberg = Command::new(env!("CARGO_BIN_EXE_heisenberg"));

    for variable in COMPILER_VARIABLES
        .into_iter()
        .chain(RUSTFLAGS_VARIABLES)
        .chain(NIGHTLY_FEATURE_VARIABLES)
        .chain([
            CLIPPY_ARGUMENTS_VARIABLE,
            CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE,
            RUSTUP_TOOLCHAIN_VARIABLE,
        ])
    {
        heisenberg.env_remove(variable);
    }

    let output = heisenberg
        .arg("check")
        .args(HOST_SELECTION)
        .arg("--cargo-home")
        .arg(&cargo_home)
        .arg("--sysroot")
        .arg(&sysroot)
        .arg("--workspace")
        .arg(
            FixtureWorkspace {
                name: "dependency_with_its_own_toolchain",
            }
            .path(),
        )
        .output()
        .expect("the heisenberg binary must start");

    assert_eq!(str::from_utf8(&output.stderr), Ok(""));
    assert_eq!(
        str::from_utf8(&output.stdout),
        Ok([
            "------------------------------\n\
             No errors found!\n\
             ------------------------------\n",
            host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(output.status.code(), Some(0));
}
