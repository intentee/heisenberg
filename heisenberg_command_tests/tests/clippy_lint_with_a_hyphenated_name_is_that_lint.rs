use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn clippy_lint_with_a_hyphenated_name_is_that_lint() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_lint_with_a_hyphenated_name",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ClippyLintNotDenied: 1 violation\n\
            Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave it undenied, in its own lints table.\n\
            \n\
            - Cargo.toml: Package clippy_lint_with_a_hyphenated_name does not deny clippy::expect_used\n\
            \n\
            LintAllowedInManifest: 1 violation\n\
            Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow; if the lint then reports code, fix that code.\n\
            \n\
            - Cargo.toml: Package clippy_lint_with_a_hyphenated_name allows expect-used in its clippy lints\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
