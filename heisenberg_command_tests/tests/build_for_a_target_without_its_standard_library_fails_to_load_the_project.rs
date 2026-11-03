use std::env;
use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::constants::SYSROOT_VARIABLE;

#[test]
fn build_for_a_target_without_its_standard_library_fails_to_load_the_project() {
    let sysroot = env::var(SYSROOT_VARIABLE).expect("the sysroot variable must be set");
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_for_a_target_without_its_standard_library",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the build scripts and procedural macros of the workspace cannot be built for aarch64-apple-darwin\n\
             error: the standard library of aarch64-apple-darwin is not installed at `{}`\n",
            Path::new(&sysroot)
                .join("lib")
                .join("rustlib")
                .join("aarch64-apple-darwin")
                .join("lib")
                .display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
