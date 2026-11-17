use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn function_used_only_by_tests_of_a_publishable_library_is_reported_only_when_unreachable_from_other_crates()
 {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "functions_used_only_by_tests_in_a_publishable_library",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemUsedOnlyByTests: 3 violations\n\
            Production code holds only what production uses. Move an item that only tests use into the test module that uses it, or into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - src/counter.rs:1:8: Struct is used only by tests\n    \
            struct Hidden;\n\
            - src/counter.rs:4:18: Function is used only by tests\n        \
            pub const fn value(&self) -> u8 {\n\
            - src/counter.rs:22:14: Function is used only by tests\n        \
            const fn doubled(&self) -> u8 {\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(3, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
