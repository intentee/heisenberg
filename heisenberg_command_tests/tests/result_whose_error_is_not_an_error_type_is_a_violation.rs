use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn result_whose_error_is_not_an_error_type_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "untyped_result_errors",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UntypedResultError: 8 violations\n\
            An error that is not an error type erases which failure happened: a caller can print it, or downcast it where it carries a type, but cannot match on it, and the signature names no failure. Declare an error enum whose variants name each failure, and return it.\n\
            \n\
            - src/boxed_failure.rs:3:41: Result error type Box<dyn Error> does not implement std::error::Error\n    \
            pub fn boxed_failure(succeeds: bool) -> Result<u8, Box<dyn Error>> {\n\
            - src/holder.rs:2:17: Result error type &'static str does not implement std::error::Error\n        \
            pub result: Result<u8, &'static str>,\n\
            - src/outcome.rs:1:20: Result error type () does not implement std::error::Error\n    \
            pub type Outcome = Result<u8, ()>;\n\
            - src/parse_number.rs:1:36: Result error type String does not implement std::error::Error\n    \
            pub fn parse_number(text: &str) -> Result<u8, String> {\n\
            - src/retry.rs:4:23: Result error type (), which the alias untyped_result_errors::outcome::Outcome names, does not implement std::error::Error\n    \
            pub fn retry(outcome: Outcome) -> TextOutcome {\n\
            - src/retry.rs:4:35: Result error type alloc::string::String, which the alias untyped_result_errors::text_outcome::TextOutcome names, does not implement std::error::Error\n    \
            pub fn retry(outcome: Outcome) -> TextOutcome {\n\
            - src/text_outcome.rs:1:24: Result error type String does not implement std::error::Error\n    \
            pub type TextOutcome = Result<u8, String>;\n\
            - src/word.rs:8:16: Result error type String does not implement std::error::Error\n        \
            type Err = String;\n\
            \n\
            ------------------------------\n\
            8 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 10).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
