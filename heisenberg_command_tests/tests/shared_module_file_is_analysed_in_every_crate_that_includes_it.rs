use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn shared_module_file_is_analysed_in_every_crate_that_includes_it() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "shared_module_file_closing_a_cycle",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: SharedModuleFile - tests/support/first_part.rs - Module file belongs to 2 modules\n  \
            crate::support::first_part in shared_module_file_closing_a_cycle test \"first\"\n  \
            crate::support::first_part in shared_module_file_closing_a_cycle test \"second\"\n\
            \n\
            ERROR: NonTestItemInTestTarget - tests/support/first_part.rs:1:1 - Integration test target holds an item that is neither a test, a constant nor an import\n\
            pub struct FirstPart;\n\
            \n\
            ERROR: NonTestItemInTestTarget - tests/support/first_part.rs:3:1 - Integration test target holds an item that is neither a test, a constant nor an import\n\
            impl FirstPart {\n\
            \n\
            ERROR: ModuleCycle - tests/support/first_part.rs:5:9 - Parts of crate::support depend on each other\n        \
            super::second_part::SecondPart::back();\n  \
            crate::support::first_part -> crate::support::second_part:\n    \
            crate::support::first_part uses crate::support::second_part at tests/support/first_part.rs:5:9\n  \
            crate::support::second_part -> crate::support::first_part:\n    \
            crate::support::second_part uses crate::support::first_part at tests/support/second_part.rs:4:29\n\
            \n\
            ERROR: SharedModuleFile - tests/support/mod.rs - Module file belongs to 2 modules\n  \
            crate::support in shared_module_file_closing_a_cycle test \"first\"\n  \
            crate::support in shared_module_file_closing_a_cycle test \"second\"\n\
            \n\
            ERROR: SharedModuleFile - tests/support/second_part.rs - Module file belongs to 2 modules\n  \
            crate::support::second_part in shared_module_file_closing_a_cycle test \"first\"\n  \
            crate::support::second_part in shared_module_file_closing_a_cycle test \"second\"\n\
            \n\
            ERROR: NonTestItemInTestTarget - tests/support/second_part.rs:1:1 - Integration test target holds an item that is neither a test, a constant nor an import\n\
            pub struct SecondPart;\n\
            \n\
            ERROR: NonTestItemInTestTarget - tests/support/second_part.rs:3:1 - Integration test target holds an item that is neither a test, a constant nor an import\n\
            impl SecondPart {\n\
            \n\
            ------------------------------\n\
            8 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 9 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 9 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
