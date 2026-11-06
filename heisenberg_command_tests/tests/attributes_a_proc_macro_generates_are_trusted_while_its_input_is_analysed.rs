use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn attributes_a_proc_macro_generates_are_trusted_while_its_input_is_analysed() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "proc_macro_generated_attributes",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["LintSuppression: 1 violation\n\
            Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.\n\
            \n\
            - src/marked.rs:4:3: Attribute suppresses lints\n    \
            #[allow(unused)]\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(2, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
