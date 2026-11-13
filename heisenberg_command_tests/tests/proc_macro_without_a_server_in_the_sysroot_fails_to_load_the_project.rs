use std::collections::BTreeMap;
use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink as symlink_directory;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink_directory;
use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::constants::SYSROOT_VARIABLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn proc_macro_without_a_server_in_the_sysroot_fails_to_load_the_project() {
    let installed = env::var_os(SYSROOT_VARIABLE).expect("the sysroot variable must be set");
    let sysroot =
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("sysroot_without_a_proc_macro_server");

    fs::create_dir_all(&sysroot)
        .and_then(|()| fs::remove_dir_all(&sysroot))
        .and_then(|()| fs::create_dir_all(&sysroot))
        .expect("the sysroot must be created afresh");

    for directory in ["bin", "lib"] {
        symlink_directory(
            Path::new(&installed).join(directory),
            sysroot.join(directory),
        )
        .expect("the sysroot must link the installed compiler and libraries");
    }

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
    let missing = format!(
        "error: the sysroot has no procedural macro server\n\
         error: cannot find proc-macro server in sysroot `{}`\n",
        sysroot.display()
    );

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded for the configuration `{HOST_CONFIGURATION}`\n\
             {missing}"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
