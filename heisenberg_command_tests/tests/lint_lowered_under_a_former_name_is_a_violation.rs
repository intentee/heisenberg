use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn lint_lowered_under_a_former_name_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lint_lowered_under_a_former_name",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["LintSuppression: 1 violation\n\
            Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.\n\
            \n\
            - src/lib.rs:1:3: Attribute suppresses lints\n    \
            #[warn(clippy::result_expect_used)]\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
