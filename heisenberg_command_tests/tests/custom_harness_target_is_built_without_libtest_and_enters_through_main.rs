use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn custom_harness_target_is_built_without_libtest_and_enters_through_main() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "custom_harness_test_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("CodeActiveOnNoTarget: 1 violation\n\
            Code active on no declared target is never built or analysed. Remove it, declare a supported target it is active on, or make it active on a declared target.\n\
            \n\
            - tests/trials.rs:16:4: Code is active on no declared target\n    \
            fn compiled_only_with_libtest() {}\n\
            \n\
            NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/trials.rs:11:8: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            pub fn shared_helper() -> u8 {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
