use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn substituted_package_that_a_build_script_depends_on_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "substituted_package_built_for_a_build_script",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the build scripts and procedural macros of the workspace cannot be built for the configuration `{HOST_CONFIGURATION}`\n\
             error: the build scripts and procedural macros of the workspace compile `native`, which needs its build script to run, and out_dirs declares its output instead of running it; a package that build scripts or procedural macros depend on cannot be declared in out_dirs\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
