use std::env;
use std::path::Path;
use std::process::Child;
use std::str;

use heisenberg_command_tests::binary_check::BinaryCheck;
use heisenberg_command_tests::constants::CARGO_HOME_VARIABLE;
use heisenberg_command_tests::constants::CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
            "PathPackageOutsideWorkspace: 1 violation\n\
             Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
             \n\
             - Cargo.toml: Package toolchain_pinned_dependency v0.1.0 is built from ../toolchain_pinned_dependency/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
             \n\
             ------------------------------\n\
             1 errors found in 1 rule\n\
             ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 1)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(output.status.code(), Some(1));
}
