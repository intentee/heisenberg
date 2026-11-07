use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn test_crate_declaration_that_does_not_hold_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "invalid_test_crate_declarations",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["PublishableTestCrate: 1 violation\n\
            A test crate holds only what tests use, so it is never published. Set publish = false in its manifest, or remove it from test_crates.\n\
            \n\
            - published/Cargo.toml: Package published is declared a test crate but can be published\n\
            \n\
            RequiredLintNotDenied: 6 violations\n\
            Every warning and every lint of the clippy groups fails the build, and production code reports failures as typed errors instead of panicking. Deny the lint in the [lints] or [workspace.lints] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave the panicking lints undenied, in its own lints table.\n\
            \n\
            - published/Cargo.toml: Package published does not deny clippy::expect_used\n\
            - published/Cargo.toml: Package published does not deny clippy::panic\n\
            - published/Cargo.toml: Package published does not deny clippy::unwrap_used\n\
            - used/Cargo.toml: Package used does not deny clippy::expect_used\n\
            - used/Cargo.toml: Package used does not deny clippy::panic\n\
            - used/Cargo.toml: Package used does not deny clippy::unwrap_used\n\
            \n\
            TestCrateDeclaredTwice: 1 violation\n\
            Each test crate is declared once. Remove the repeated entry from test_crates.\n\
            \n\
            - Cargo.toml: Package used is declared a test crate more than once\n\
            \n\
            TestCrateNotAMember: 1 violation\n\
            Only a workspace member can be a test crate. Remove the entry from test_crates, or add the package to the workspace.\n\
            \n\
            - Cargo.toml: Package missing is declared a test crate but is not a workspace member\n\
            \n\
            TestCrateUsedByProduction: 1 violation\n\
            Production code never depends on test code. Make the dependency a dev-dependency, or remove the package from test_crates.\n\
            \n\
            - library/Cargo.toml: Package library depends on test crate used through a normal or build dependency\n\
            \n\
            ------------------------------\n\
            10 errors found\n\
            ------------------------------\n", host_footer(3, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
