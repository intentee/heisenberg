use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

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
        Ok("FunctionUsedOnlyByTests: 2 violations\n\
            Production code holds only what production uses. Move a function that only tests use into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - src/counter.rs:14:14: Function is used only by tests\n        \
            const fn doubled(&self) -> u8 {\n\
            - src/counter.rs:22:18: Function is used only by tests\n        \
            pub const fn value(&self) -> u8 {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 6 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 6 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
