use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn single_letter_parameter_names_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "single_letter_names",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["SingleLetterName: 7 violations\n\
            A single character says nothing about what a name stands for. Name it after its meaning, such as 'database for a lifetime or TNode for a type parameter.\n\
            \n\
            - src/borrowed.rs:1:21: Name is a single character\n    \
            pub struct Borrowed<'a, T> {\n\
            - src/borrowed.rs:1:25: Name is a single character\n    \
            pub struct Borrowed<'a, T> {\n\
            - src/callback.rs:1:25: Name is a single character\n    \
            pub type Callback = for<'b> fn(&'b str) -> &'b str;\n\
            - src/fixed.rs:1:24: Name is a single character\n    \
            pub struct Fixed<const N: usize>;\n\
            - src/sum.rs:1:12: Name is a single character\n    \
            pub fn sum(x: u8, values: &[u8]) -> u8 {\n\
            - src/sum.rs:2:36: Name is a single character\n        \
            values.iter().fold(x, |total, &v| total + v) + values.iter().map(|y| y + 1).sum::<u8>()\n\
            - src/sum.rs:2:71: Name is a single character\n        \
            values.iter().fold(x, |total, &v| total + v) + values.iter().map(|y| y + 1).sum::<u8>()\n\
            \n\
            ------------------------------\n\
            7 errors found\n\
            ------------------------------\n", host_footer(1, 5).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
