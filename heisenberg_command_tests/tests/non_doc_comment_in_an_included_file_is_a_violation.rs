use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn non_doc_comment_in_an_included_file_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "non_doc_comment_in_an_included_file",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NonDocComment: 1 violation\n\
            Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an API.\n\
            \n\
            - snippets/included.rs:1:1: Comment is not a doc comment\n    \
            // A comment in an included file.\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
