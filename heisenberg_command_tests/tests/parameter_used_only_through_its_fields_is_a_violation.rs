use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn parameter_used_only_through_its_fields_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "undestructured_parameters",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UndestructuredParameter: 4 violations\n\
            A parameter used only through its fields reads better as a pattern that names those fields. Destructure the parameter in its pattern.\n\
            \n\
            - src/meters.rs:4:26: Parameter is used only through the fields of Meters\n        \
            pub fn longer(&self, other: &Self) -> bool {\n\
            - src/total.rs:5:15: Parameter is used only through the fields of (usize, &u8)\n            \
            .map(|entry| entry.0)\n\
            - src/width.rs:3:14: Parameter is used only through the fields of Range<u8>\n    \
            pub fn width(range: Range<u8>, ranges: &[Range<u8>]) -> u8 {\n\
            - src/width.rs:6:15: Parameter is used only through the fields of Range<u8>\n            \
            .map(|other| other.end)\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n", host_footer(1, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
