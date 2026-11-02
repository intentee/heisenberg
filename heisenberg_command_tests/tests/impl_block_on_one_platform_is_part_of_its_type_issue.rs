use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn impl_block_on_one_platform_is_part_of_its_type_issue() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "impl_block_of_a_flagged_type_on_one_platform",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: NonTestItemInTestTarget - tests/recording.rs:1:8 - Integration test target holds an item that is neither a test, a constant nor an import\n\
            struct Recorder;\n  \
            impl block at tests/recording.rs:4:6\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
