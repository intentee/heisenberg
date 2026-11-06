use std::collections::BTreeMap;
use std::env::consts::EXE_EXTENSION;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn build_with_a_sysroot_without_rustc_fails_to_load_the_project() {
    let sysroot = FixtureWorkspace {
        name: "process_free_check",
    }
    .path()
    .join("sysroot");
    let outcome = CheckOutcome::of(
        &HeisenbergCheck {
            workspace: &FixtureWorkspace {
                name: "feature_seen_by_its_build_script",
            }
            .path(),
        }
        .command(&sysroot)
        .expect("the arguments must parse"),
        &BTreeMap::new(),
    );

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the sysroot has no rustc at `{}`\n",
            sysroot
                .join("bin")
                .join("rustc")
                .with_extension(EXE_EXTENSION)
                .display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
