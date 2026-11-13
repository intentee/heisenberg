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
            "UntypedResultError: 5 violations\n\
            An error that is not an error type carries no source chain and no variant that a caller can match, so callers can only print it or compare its text. Declare an error enum whose variants name each failure, and return it.\n\
            \n\
            - src/boxed_failure.rs:3:41: Result error type Box<dyn Error + 'static> does not implement std::error::Error\n    \
            pub fn boxed_failure(succeeds: bool) -> Result<u8, Box<dyn Error>> {\n\
            - src/holder.rs:2:17: Result error type &'static str does not implement std::error::Error\n        \
            pub result: Result<u8, &'static str>,\n\
            - src/outcome.rs:1:20: Result error type () does not implement std::error::Error\n    \
            pub type Outcome = Result<u8, ()>;\n\
            - src/parse_number.rs:1:36: Result error type String does not implement std::error::Error\n    \
            pub fn parse_number(text: &str) -> Result<u8, String> {\n\
            - src/word.rs:8:16: Result error type String does not implement std::error::Error\n        \
            type Err = String;\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 8).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
