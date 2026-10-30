use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: NonConstantInConstantsModule - src/constants.rs:5:1 - Constants module holds a public item that is not a constant\n\
            pub struct Unit;\n\
            \n\
            ERROR: NonConstantInConstantsModule - src/constants.rs:7:1 - Constants module holds a public item that is not a constant\n\
            pub fn helper() -> u8 {\n\
            \n\
            ERROR: NonConstantInConstantsModule - src/constants.rs:11:1 - Constants module holds a public item that is not a constant\n\
            pub static COUNTER: u8 = 0;\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
