use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn result_error_that_is_a_workspace_type_but_not_an_error_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "result_errors_that_are_not_errors",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ResultErrorNotAnError: 3 violations\n\
            Err is reserved for failures. A value that is not an error type is either an outcome of the regular flow handed back through Err, which hides an expected result where failures belong, or a failure without the contract of an error, its message and its source chain. When the value is a failure, make it the error type of its crate, or a variant of it, deriving thiserror::Error. When it is an outcome of the regular flow, return it in Ok, as a variant of an enum of outcomes.\n\
            \n\
            - src/decision.rs:3:21: Result error type Rejection is a type of this workspace that is not an error\n    \
            pub type Decision = Result<usize, Rejection>;\n\
            - src/holder.rs:4:17: Result error type Rejection is a type of this workspace that is not an error\n        \
            pub result: Result<usize, Rejection>,\n\
            - src/parsed.rs:3:30: Result error type Rejection is a type of this workspace that is not an error\n    \
            pub fn parsed(text: &str) -> Result<usize, Rejection> {\n\
            \n\
            UntypedResultError: 1 violation\n\
            An error type that erases which failure happened names no failure in the signature: a caller can print it, or downcast it where it carries a type, but cannot match on it. Declare an error enum whose variants name each failure, and return it. Return several failures as a Vec of that error type.\n\
            \n\
            - src/collected.rs:1:37: Result error type Vec<String> does not implement std::error::Error\n    \
            pub fn collected(texts: &[&str]) -> Result<usize, Vec<String>> {\n\
            \n\
            ------------------------------\n\
            4 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 7).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
