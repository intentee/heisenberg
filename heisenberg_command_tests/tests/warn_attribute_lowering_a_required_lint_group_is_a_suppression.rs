use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn warn_attribute_lowering_a_required_lint_group_is_a_suppression() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lowered_lint_groups",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["LintSuppression: 2 violations\n\
            Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.\n\
            \n\
            - src/lib.rs:4:3: Attribute suppresses lints\n    \
            #[warn(clippy::pedantic)]\n\
            - src/lib.rs:7:3: Attribute suppresses lints\n    \
            #[warn(warnings)]\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
