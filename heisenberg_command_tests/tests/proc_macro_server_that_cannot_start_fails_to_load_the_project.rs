use std::collections::BTreeMap;
use std::env;
use std::env::consts::EXE_SUFFIX;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink as symlink_directory;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink_directory;
use std::path::Path;
use std::process::Command;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn proc_macro_server_that_cannot_start_fails_to_load_the_project() {
    let installed = env::var_os(SYSROOT_VARIABLE).expect("the sysroot variable must be set");
    let sysroot = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("sysroot_with_an_unstartable_proc_macro_server");
    let server = sysroot
        .join("libexec")
        .join(format!("rust-analyzer-proc-macro-srv{EXE_SUFFIX}"));

    fs::create_dir_all(&sysroot)
        .and_then(|()| fs::remove_dir_all(&sysroot))
        .and_then(|()| fs::create_dir_all(sysroot.join("libexec")))
        .and_then(|()| fs::write(&server, b""))
        .expect("the sysroot must be created afresh");

    for directory in ["bin", "lib"] {
        symlink_directory(
            Path::new(&installed).join(directory),
            sysroot.join(directory),
        )
        .expect("the sysroot must link the installed compiler and libraries");
    }

    let refused = Command::new(&server)
        .spawn()
        .err()
        .map(|error| error.to_string());
    let outcome = CheckOutcome::of(
        &HeisenbergCheck {
            workspace: &FixtureWorkspace {
                name: "function_like_proc_macro_in_expression_position",
            }
            .path(),
        }
        .command(&sysroot)
        .expect("the arguments must parse"),
        &BTreeMap::new(),
    )
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&outcome.errors).map(ToOwned::to_owned),
        Ok(refused.map_or_else(String::new, |reason| {
            format!(
                "error: the project cannot be loaded for the configuration `{HOST_CONFIGURATION}`\n\
                 error: the procedural macro server `{}` cannot be started\n\
                 error: {reason}\n",
                server.display()
            )
        }))
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
