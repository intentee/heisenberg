use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn single_letter_lifetime_in_proc_macro_attribute_arguments_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lifetime_in_proc_macro_attribute_arguments",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["SingleLetterName: 1 violation\n\
            A single character says nothing about what a name stands for. Name it after its meaning, such as 'database for a lifetime or TNode for a type parameter.\n\
            \n\
            - src/holder.rs:1:23: Name is a single character\n    \
            #[identity::emit(impl<'b> From<&'b str> for Holder { fn from(_name: &'b str) -> Self { Self } })]\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(2, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
