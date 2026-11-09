use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn test_crate_leaving_panicking_lints_undenied_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "test_crate_leaving_panicking_lints_undenied",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RequiredLintNotDenied: 10 violations\n\
            Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items. Deny the lint in the [lints] or [workspace.lints] table the package uses.\n\
            \n\
            - checks/Cargo.toml: Package checks does not deny clippy::arithmetic_side_effects\n\
            - checks/Cargo.toml: Package checks does not deny clippy::expect_used\n\
            - checks/Cargo.toml: Package checks does not deny clippy::indexing_slicing\n\
            - checks/Cargo.toml: Package checks does not deny clippy::panic\n\
            - checks/Cargo.toml: Package checks does not deny clippy::panic_in_result_fn\n\
            - checks/Cargo.toml: Package checks does not deny clippy::string_slice\n\
            - checks/Cargo.toml: Package checks does not deny clippy::todo\n\
            - checks/Cargo.toml: Package checks does not deny clippy::unimplemented\n\
            - checks/Cargo.toml: Package checks does not deny clippy::unreachable\n\
            - checks/Cargo.toml: Package checks does not deny clippy::unwrap_used\n\
            \n\
            ------------------------------\n\
            10 errors found\n\
            ------------------------------\n", host_footer(3, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
