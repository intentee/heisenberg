use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn build_script_needing_a_substituted_build_script_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_needing_a_substituted_build_script",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: the build scripts and procedural macros of the workspace cannot be built for x86_64-pc-windows-msvc\n\
            error: the build script of `wrapper` needs the build script of `native`, which out_dirs declares instead of running it\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
