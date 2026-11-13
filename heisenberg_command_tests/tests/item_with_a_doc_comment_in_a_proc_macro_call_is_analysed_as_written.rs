use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_with_a_doc_comment_in_a_proc_macro_call_is_analysed_as_written() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "doc_commented_item_in_a_proc_macro_call",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["AdditionalPublicItem: 1 violation\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - src/shapes.rs:3:16: Module already has a public item\n        \
            pub struct Second;\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
