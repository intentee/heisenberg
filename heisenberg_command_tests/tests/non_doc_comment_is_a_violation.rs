use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn non_doc_comment_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "non_doc_comments",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NonDocComment: 5 violations\n\
            Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed, and only on the items they document. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an item; a statement, an expression or an unsafe block has no comment form, and unsafe code exists only where a crate integrates foreign code.\n\
            \n\
            - src/lib.rs:6:1: Comment is not a doc comment\n    \
            // A line comment explains what names should.\n\
            - src/lib.rs:7:1: Comment is not a doc comment\n    \
            /* A block comment explains what names should. */\n\
            - src/lib.rs:11:5: Comment is not a doc comment\n        \
            // A comment in code active on one target.\n\
            - src/lib.rs:17:5: Comment is not a doc comment\n        \
            // A comment in code active on the other target.\n\
            - src/lib.rs:23:9: Comment is not a doc comment\n            \
            // A comment in a macro body.\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
