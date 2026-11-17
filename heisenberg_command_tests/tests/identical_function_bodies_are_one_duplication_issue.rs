use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn identical_function_bodies_are_one_duplication_issue() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "identical_function_bodies",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["DuplicatedFunctionBody: 1 violation\n\
            Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.\n\
            \n\
            - src/first.rs:1:8: Function body is duplicated\n    \
            pub fn first(names: &[String]) -> usize {\n      \
            duplicate at src/second.rs:1:8\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
