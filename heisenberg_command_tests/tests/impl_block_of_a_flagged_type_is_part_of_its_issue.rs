use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn impl_block_of_a_flagged_type_is_part_of_its_issue() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "impl_block_of_a_flagged_type",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/helping.rs:3:16: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n        \
            pub struct Helper;\n      \
            impl block at tests/helping.rs:6:6\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
