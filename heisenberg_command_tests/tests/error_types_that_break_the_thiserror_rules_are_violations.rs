use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn error_types_that_break_the_thiserror_rules_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "error_types",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ErrorMessageOnTheEnum: 1 violation\n\
            Each failure has its own message, written on its variant. Move the message to the variants, one #[error] attribute each.\n\
            \n\
            - several/src/forwarded_failure.rs:5:1: Error message is written on the enum instead of its variants\n    \
            #[error(transparent)]\n\
            \n\
            ErrorSourceOutsideChain: 6 violations\n\
            An error kept outside the source never reaches the error chain, so whoever reports the error loses its cause. Make the field the source of its variant, with #[source] or #[from]. Where a failure has several causes, keep one as the source and format each other one in the message of the variant.\n\
            \n\
            - several/src/forwarded_failure.rs:9:11: Field holds an error outside the source chain of its variant\n        \
            Empty(ParseIntError),\n\
            - single/src/lookup_failure.rs:47:9: Field holds an error outside the source chain of its variant\n            \
            first: ParseIntError,\n\
            - single/src/lookup_failure.rs:52:15: Field holds an error outside the source chain of its variant\n        \
            Context { context: io::Error },\n\
            - single/src/lookup_failure.rs:62:14: Field holds an error outside the source chain of its variant\n        \
            Reason { reason: io::Error },\n\
            - single/src/lookup_failure.rs:64:16: Field holds an error outside the source chain of its variant\n        \
            Optional { cause: Option<io::Error> },\n\
            - single/src/lookup_failure.rs:66:14: Field holds an error outside the source chain of its variant\n        \
            Erased { cause: Box<dyn Error + Send + Sync> },\n\
            \n\
            ErrorTypeNotAnEnum: 1 violation\n\
            A type that is not an enum cannot name which failure happened, so its callers cannot match on the failure. Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.\n\
            \n\
            - several/src/read_failure.rs:1:3: Error type that derives thiserror::Error is not an enum\n    \
            #[derive(Debug, thiserror::Error)]\n\
            \n\
            HandwrittenErrorDisplay: 2 violations\n\
            A message written in a Display implementation escapes the rules that check error messages, and is not read where the variant is declared. Remove the implementation and give each variant an #[error] attribute.\n\
            \n\
            - several/src/closed_failure.rs:11:18: Error type formats its message by hand\n    \
            impl Display for ClosedFailure {\n\
            - several/src/declared_failure.rs:19:18: Error type formats its message by hand\n    \
            impl Display for DeclaredFailure {\n\
            \n\
            HandwrittenErrorType: 1 violation\n\
            An error type written by hand repeats what thiserror generates, its messages, source chain and conversions, and drifts from it. Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.\n\
            \n\
            - several/src/closed_failure.rs:17:16: Error type is implemented by hand\n    \
            impl Error for ClosedFailure {}\n\
            \n\
            MacroGeneratedErrorType: 1 violation\n\
            The error type rules read the source and the message of each failure from what the Error derive of thiserror declares, and another macro declares neither where they can be checked. Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.\n\
            \n\
            - several/src/declared_failure.rs:12:13: Error type is implemented by a macro other than thiserror::Error\n    \
            error_type!(DeclaredFailure);\n\
            \n\
            MultipleErrorTypes: 5 violations\n\
            A crate whose failures one error enum cannot describe does more than one thing, and its callers cannot tell from its signatures which concern failed. Split the crate along its error boundaries: move each error enum, with the code that fails with it, into its own crate, and wrap it in the crates that depend on it as a #[source] or #[from] variant of their own error enum.\n\
            \n\
            - several/src/closed_failure.rs:17:16: Error type is one of the 5 that the crate implements\n    \
            impl Error for ClosedFailure {}\n\
            - several/src/declared_failure.rs:12:13: Error type is one of the 5 that the crate implements\n    \
            error_type!(DeclaredFailure);\n\
            - several/src/forwarded_failure.rs:4:3: Error type is one of the 5 that the crate implements\n    \
            #[derive(Debug, thiserror::Error)]\n\
            - several/src/parse_failure.rs:1:3: Error type is one of the 5 that the crate implements\n    \
            #[derive(Debug, thiserror::Error)]\n\
            - several/src/read_failure.rs:1:3: Error type is one of the 5 that the crate implements\n    \
            #[derive(Debug, thiserror::Error)]\n\
            \n\
            SourceInErrorMessage: 5 violations\n\
            The error chain reports the source after the message, so a message that formats the source as well prints the cause twice. Leave the source out of the message and let the error chain report it.\n\
            \n\
            - single/src/lookup_failure.rs:19:13: Error message formats the source that the error chain reports after it\n        \
            #[error(\"the value cannot be parsed: {source}\")]\n\
            - single/src/lookup_failure.rs:21:13: Error message formats the source that the error chain reports after it\n        \
            #[error(\"the limit is exceeded: {0:?}\")]\n\
            - single/src/lookup_failure.rs:23:47: Error message formats the source that the error chain reports after it\n        \
            #[error(\"the entry cannot be parsed: {}\", &.source)]\n\
            - single/src/lookup_failure.rs:25:47: Error message formats the source that the error chain reports after it\n        \
            #[error(\"the field cannot be parsed: {}\", (.source))]\n\
            - single/src/lookup_failure.rs:29:47: Error message formats the source that the error chain reports after it\n        \
            #[error(\"the text cannot be parsed: {}\", .source.to_string())]\n\
            \n\
            ------------------------------\n\
            22 errors found in 8 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(2, 8).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
