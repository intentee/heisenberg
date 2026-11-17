use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn lint_group_left_undenied_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lint_groups_not_denied",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RequiredLintNotDenied: 4 violations\n\
            Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items. Deny the lint in the [lints] or [workspace.lints] table the package uses.\n\
            \n\
            - Cargo.toml: Package lint_groups_not_denied does not deny clippy::all\n\
            - Cargo.toml: Package lint_groups_not_denied does not deny clippy::nursery\n\
            - Cargo.toml: Package lint_groups_not_denied does not deny clippy::pedantic\n\
            - Cargo.toml: Package lint_groups_not_denied does not deny warnings\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
