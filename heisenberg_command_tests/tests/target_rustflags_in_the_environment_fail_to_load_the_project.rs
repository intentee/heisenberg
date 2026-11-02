use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn target_rustflags_in_the_environment_fail_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "acyclic_modules",
        }
        .path(),
    }
    .run_in(&BTreeMap::from([(
        OsString::from("CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS"),
        OsString::from("--cfg=probe"),
    )]))
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: `target.x86_64-unknown-linux-gnu.rustflags` is set in environment variable `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`, and Heisenberg cannot account for rustflags\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
