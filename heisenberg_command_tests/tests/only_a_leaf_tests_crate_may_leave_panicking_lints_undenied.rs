use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn only_a_leaf_tests_crate_may_leave_panicking_lints_undenied() {
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
        Ok("ClippyLintNotDenied: 3 violations\n\
            Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses; only a package named <something>_tests that no workspace member depends on may leave it undenied, in its own lints table.\n\
            \n\
            - shared_tests/Cargo.toml: Package shared_tests does not deny clippy::expect_used\n\
            - shared_tests/Cargo.toml: Package shared_tests does not deny clippy::panic\n\
            - shared_tests/Cargo.toml: Package shared_tests does not deny clippy::unwrap_used\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 4 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 4 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
