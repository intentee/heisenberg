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
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 3 crate targets, 9 modules\n\
            analysed on x86_64-unknown-linux-gnu: 3 crate targets, 9 modules\n\
            cycle in shared_module_file_closing_a_cycle test \"first\" among the parts of crate::support (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::support::first_part -> crate::support::second_part:\n    \
            crate::support::first_part uses crate::support::second_part at tests/support/first_part.rs:5:9\n  \
            crate::support::second_part -> crate::support::first_part:\n    \
            crate::support::second_part uses crate::support::first_part at tests/support/second_part.rs:4:29\n\
            cycle in shared_module_file_closing_a_cycle test \"second\" among the parts of crate::support (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::support::first_part -> crate::support::second_part:\n    \
            crate::support::first_part uses crate::support::second_part at tests/support/first_part.rs:5:9\n  \
            crate::support::second_part -> crate::support::first_part:\n    \
            crate::support::second_part uses crate::support::first_part at tests/support/second_part.rs:4:29\n\
            violation in shared_module_file_closing_a_cycle test \"first\": integration test target holds an item that is neither a test nor an import at tests/support/first_part.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"first\": integration test target holds an item that is neither a test nor an import at tests/support/first_part.rs:3:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"first\": integration test target holds an item that is neither a test nor an import at tests/support/second_part.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"first\": integration test target holds an item that is neither a test nor an import at tests/support/second_part.rs:3:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"second\": integration test target holds an item that is neither a test nor an import at tests/support/first_part.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"second\": integration test target holds an item that is neither a test nor an import at tests/support/first_part.rs:3:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"second\": integration test target holds an item that is neither a test nor an import at tests/support/second_part.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in shared_module_file_closing_a_cycle test \"second\": integration test target holds an item that is neither a test nor an import at tests/support/second_part.rs:3:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation: module file tests/support/first_part.rs belongs to 2 modules (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::support::first_part in shared_module_file_closing_a_cycle test \"first\"\n  \
            crate::support::first_part in shared_module_file_closing_a_cycle test \"second\"\n\
            violation: module file tests/support/mod.rs belongs to 2 modules (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::support in shared_module_file_closing_a_cycle test \"first\"\n  \
            crate::support in shared_module_file_closing_a_cycle test \"second\"\n\
            violation: module file tests/support/second_part.rs belongs to 2 modules (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::support::second_part in shared_module_file_closing_a_cycle test \"first\"\n  \
            crate::support::second_part in shared_module_file_closing_a_cycle test \"second\"\n\
            result: 2 cycles, 11 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
