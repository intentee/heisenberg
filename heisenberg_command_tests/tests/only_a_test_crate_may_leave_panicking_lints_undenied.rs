use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn only_a_test_crate_may_leave_panicking_lints_undenied() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tests_crates_that_may_panic",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ClippyLintNotDenied: 6 violations\n\
            Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave it undenied, in its own lints table.\n\
            \n\
            - shared_tests/Cargo.toml: Package shared_tests does not deny clippy::expect_used\n\
            - shared_tests/Cargo.toml: Package shared_tests does not deny clippy::panic\n\
            - shared_tests/Cargo.toml: Package shared_tests does not deny clippy::unwrap_used\n\
            - tool/Cargo.toml: Package tool does not deny clippy::expect_used\n\
            - tool/Cargo.toml: Package tool does not deny clippy::panic\n\
            - tool/Cargo.toml: Package tool does not deny clippy::unwrap_used\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n", host_footer(5, 5).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
