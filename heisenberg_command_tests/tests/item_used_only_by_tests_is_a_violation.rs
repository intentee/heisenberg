use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_used_only_by_tests_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "items_used_only_by_tests",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemUsedOnlyByTests: 8 violations\n\
            Production code holds only what production uses. Move an item that only tests use into the test module that uses it, or into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - src/constants.rs:3:11: Constant is used only by tests\n    \
            pub const TEST_LIMIT: u8 = 9;\n\
            - src/gauge.rs:6:9: Field is read only by tests\n        \
            pub observed: u8,\n\
            - src/level.rs:4:5: Variant is constructed only by tests\n        \
            Probed,\n\
            - src/marker.rs:1:10: Enum is used only by tests\n    \
            pub enum Marker {\n\
            - src/probe.rs:1:12: Struct is used only by tests\n    \
            pub struct Probe;\n\
            - src/raw.rs:1:11: Union is used only by tests\n    \
            pub union Raw {\n\
            - src/reading.rs:1:10: Type alias is used only by tests\n    \
            pub type Reading = u8;\n\
            - src/registry.rs:1:12: Static is used only by tests\n    \
            pub static REGISTRY: u8 = 1;\n\
            \n\
            ------------------------------\n\
            8 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(1, 12).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
