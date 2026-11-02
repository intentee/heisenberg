use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("NonDocComment: 5 violations\n\
            Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an API.\n\
            \n\
            - src/lib.rs:9:1: Comment is not a doc comment\n    \
            // A line comment explains what names should.\n\
            - src/lib.rs:10:1: Comment is not a doc comment\n    \
            /* A block comment explains what names should. */\n\
            - src/lib.rs:14:5: Comment is not a doc comment\n        \
            // A comment in code active on one target.\n\
            - src/lib.rs:20:5: Comment is not a doc comment\n        \
            // A comment in code active on the other target.\n\
            - src/lib.rs:26:9: Comment is not a doc comment\n            \
            // A comment in a macro body.\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
