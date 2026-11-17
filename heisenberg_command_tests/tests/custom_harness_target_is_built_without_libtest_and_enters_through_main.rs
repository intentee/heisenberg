use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn custom_harness_target_is_built_without_libtest_and_enters_through_main() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "custom_harness_test_target",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["CodeActiveInNoConfiguration: 1 violation\n\
            Code active in no declared configuration is never built or analysed. Remove it, declare a configuration it is active in, or make it active in a declared configuration.\n\
            \n\
            - tests/trials.rs:12:4: Code is active in no declared configuration\n    \
            fn compiled_only_with_libtest() {}\n\
            \n\
            NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/trials.rs:19:8: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            pub fn shared_helper() -> u8 {\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", full_run_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
