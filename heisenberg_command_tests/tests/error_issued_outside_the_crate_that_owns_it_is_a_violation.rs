use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn error_issued_outside_the_crate_that_owns_it_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "errors_issued_outside_their_crate",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ErrorIssuedOutsideItsCrate: 8 violations\n\
            The error type of a crate is its boundary: it names the failures of one concern, and only the code of that concern knows when one of them has happened. A crate that returns or constructs the error type of another crate reports failures of a concern it does not own, so the type no longer tells which crate failed. Handle the error where it is received, or wrap it as the #[source] or #[from] of a variant of the error type of this crate and return that.\n\
            \n\
            - owner/src/main.rs:4:14: Result error type OwnerError belongs to the crate owner, and only that crate returns it\n    \
            fn main() -> Result<(), OwnerError> {\n\
            - user/src/boxed.rs:4:29: Result error type Box<OwnerError> belongs to the crate owner, and only that crate returns it\n    \
            pub fn boxed(text: &str) -> Result<u8, Box<OwnerError>> {\n\
            - user/src/collected.rs:4:33: Result error type Vec<OwnerError> belongs to the crate owner, and only that crate returns it\n    \
            pub fn collected(text: &str) -> Result<u8, Vec<OwnerError>> {\n\
            - user/src/forwarded.rs:4:33: Result error type OwnerError belongs to the crate owner, and only that crate returns it\n    \
            pub fn forwarded(text: &str) -> Result<u8, OwnerError> {\n\
            - user/src/missing.rs:5:5: Error OwnerError::Missing belongs to the crate owner, and only that crate constructs it\n        \
            OwnerError::Missing\n\
            - user/src/rejected.rs:5:5: Error OwnerError::Rejected belongs to the crate owner, and only that crate constructs it\n        \
            OwnerError::Rejected(entry)\n\
            - user/src/through_alias.rs:5:37: Result error type owner::owner_error::OwnerError, which the alias user::aliased::Aliased names, belongs to the crate owner, and only that crate returns it\n    \
            pub fn through_alias(text: &str) -> Aliased {\n\
            - user/src/unparsable.rs:7:5: Error OwnerError::Unparsable belongs to the crate owner, and only that crate constructs it\n        \
            OwnerError::Unparsable { key, source }\n\
            \n\
            ------------------------------\n\
            8 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(3, 13).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
