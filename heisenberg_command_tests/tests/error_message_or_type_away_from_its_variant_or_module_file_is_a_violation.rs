use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn error_message_or_type_away_from_its_variant_or_module_file_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "error_messages_and_modules",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ErrorMessageFunction: 1 violation\n\
            The message of an error variant is read where the variant is declared, and a function hides it from the rules that check messages. Write the message as a format string in the #[error] attribute of the variant.\n\
            \n\
            - message_function/src/formatted_failure.rs:10:5: Error message is formatted by a function\n        \
            #[error(fmt = missing)]\n\
            \n\
            ErrorMessageOnTheEnum: 1 violation\n\
            Each failure has its own message, written on its variant. Move the message to the variants, one #[error] attribute each.\n\
            \n\
            - enum_message/src/fallback_failure.rs:2:1: Error message is written on the enum instead of its variants\n    \
            #[error(\"the lookup failed\")]\n\
            \n\
            ErrorTypeOutsideItsModule: 3 violations\n\
            The error type of a crate lives in a module file of its own, found from its name alone. Move the enum into a module file named after it.\n\
            \n\
            - inline_module/src/lib.rs:3:14: Error type is defined in crate::inline_failure instead of a module file named after it\n        \
            pub enum InlineFailure {\n\
            - misnamed_module/src/failures.rs:2:6: Error type is defined in crate::failures instead of a module file named after it\n    \
            enum LookupFailure {\n\
            - root_module/src/lib.rs:2:10: Error type is defined in crate instead of a module file named after it\n    \
            pub enum RootFailure {\n\
            \n\
            HandwrittenErrorDisplay: 1 violation\n\
            A message written in a Display implementation escapes the rules that check error messages, and is not read where the variant is declared. Remove the implementation and give each variant an #[error] attribute.\n\
            \n\
            - handwritten_display/src/written_failure.rs:10:18: Error type formats its message by hand\n    \
            impl Display for WrittenFailure {\n\
            \n\
            ------------------------------\n\
            6 errors found in 4 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(7, 13).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
