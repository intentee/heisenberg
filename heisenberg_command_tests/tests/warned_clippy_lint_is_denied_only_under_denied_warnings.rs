use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn warned_clippy_lint_is_denied_only_under_denied_warnings() {
    let denied = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_lint_warned_under_denied_warnings",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");
    let warned = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_lint_warned_under_warned_warnings",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&denied.output),
        Ok([
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 1)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(denied.status, HeisenbergExitStatus::Compliant);
    assert_eq!(
        str::from_utf8(&warned.output),
        Ok(["RequiredLintNotDenied: 2 violations\n\
            Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items. Deny the lint in the [lints] or [workspace.lints] table the package uses.\n\
            \n\
            - Cargo.toml: Package clippy_lint_warned_under_warned_warnings does not deny clippy::expect_used\n\
            - Cargo.toml: Package clippy_lint_warned_under_warned_warnings does not deny warnings\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(warned.status, HeisenbergExitStatus::Violations);
}
