use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn non_constant_public_item_in_the_constants_module_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "non_constant_in_the_constants_module",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NonConstantInConstantsModule: 3 violations\n\
            The constants module is the single home of public constants. Move every other public item out of it.\n\
            \n\
            - src/constants.rs:9:12: Constants module holds a public item that is not a constant\n    \
            pub struct Unit;\n\
            - src/constants.rs:11:8: Constants module holds a public item that is not a constant\n    \
            pub fn helper() -> u8 {\n\
            - src/constants.rs:15:12: Constants module holds a public item that is not a constant\n    \
            pub static COUNTER: u8 = 0;\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n", host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
