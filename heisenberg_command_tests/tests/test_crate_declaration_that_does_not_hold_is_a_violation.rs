use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
            TestOnlyCodeInProduction: 2 violations\n\
            Production code holds only what production runs. Keep test-only code in a #[cfg(test)] module, such as the inline mod tests of the file it tests, and move test libraries shared by several packages into a test crate.\n\
            \n\
            - published/src/lib.rs:2:10: Code exists only under cfg(test) outside a test module\n    \
            const fn sample() -> u8 {\n\
            - used/src/lib.rs:2:10: Code exists only under cfg(test) outside a test module\n    \
            const fn sample() -> u8 {\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(3, 5).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
