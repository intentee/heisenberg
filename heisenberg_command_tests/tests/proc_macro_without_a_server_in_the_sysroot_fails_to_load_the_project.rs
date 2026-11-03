use std::collections::BTreeMap;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn proc_macro_without_a_server_in_the_sysroot_fails_to_load_the_project() {
    let sysroot = FixtureWorkspace {
        name: "process_free_check",
    }
    .path()
    .join("sysroot");
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
    );
    let missing = format!(
        "error: the sysroot has no procedural macro server\n\
         error: cannot find proc-macro server in sysroot `{}`\n",
        sysroot.display()
    );

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded for x86_64-pc-windows-msvc\n\
             {missing}\
             error: the project cannot be loaded for x86_64-unknown-linux-gnu\n\
             {missing}"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
