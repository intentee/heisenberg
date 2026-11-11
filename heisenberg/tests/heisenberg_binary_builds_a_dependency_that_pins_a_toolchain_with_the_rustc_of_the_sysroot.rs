use std::env;
use std::path::Path;
use std::process::Child;
use std::str;

use heisenberg_command_tests::binary_check::BinaryCheck;
use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn heisenberg_binary_builds_a_dependency_that_pins_a_toolchain_with_the_rustc_of_the_sysroot() {
    let cargo_home =
        env::var_os(CARGO_HOME_VARIABLE).expect("HEISENBERG_CARGO_HOME must be exported");
    let sysroot = env::var_os(SYSROOT_VARIABLE).expect("HEISENBERG_SYSROOT must be exported");
    let output = BinaryCheck {
        binary: Path::new(env!("CARGO_BIN_EXE_heisenberg")),
        cargo_home: Path::new(&cargo_home),
        subcommand: CHECK_SUBCOMMAND,
        sysroot: Path::new(&sysroot),
        workspace: &FixtureWorkspace {
            name: "dependency_with_its_own_toolchain",
        }
        .path(),
    }
    .spawn_with_toolchain_environment(&[])
    .and_then(Child::wait_with_output)
    .expect("the heisenberg binary must run");

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
