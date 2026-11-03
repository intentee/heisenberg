use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - tests/support/first_part.rs:5:9: Parts of crate::support depend on each other\n            \
            super::second_part::SecondPart::back();\n      \
            crate::support::first_part -> crate::support::second_part:\n        \
            crate::support::first_part uses crate::support::second_part at tests/support/first_part.rs:5:9\n      \
            crate::support::second_part -> crate::support::first_part:\n        \
            crate::support::second_part uses crate::support::first_part at tests/support/second_part.rs:4:29\n\
            \n\
            NonTestItemInTestTarget: 2 violations\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/support/first_part.rs:1:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            pub struct FirstPart;\n      \
            impl block at tests/support/first_part.rs:3:6\n\
            - tests/support/second_part.rs:1:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            pub struct SecondPart;\n      \
            impl block at tests/support/second_part.rs:3:6\n\
            \n\
            SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/support/mod.rs: Module file belongs to 2 modules\n      \
            crate::support in shared_module_file_closing_a_cycle test \"first\"\n      \
            crate::support in shared_module_file_closing_a_cycle test \"second\"\n      \
            files of its submodules:\n        \
            tests/support/first_part.rs\n        \
            tests/support/second_part.rs\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 9 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 9 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
