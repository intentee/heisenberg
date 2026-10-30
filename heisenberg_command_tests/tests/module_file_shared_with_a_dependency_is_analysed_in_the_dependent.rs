use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn module_file_shared_with_a_dependency_is_analysed_in_the_dependent() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "binary_sharing_a_module_file_with_its_library",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 2 crate targets, 4 modules\n\
            analysed on x86_64-unknown-linux-gnu: 2 crate targets, 4 modules\n\
            cycle in binary_sharing_a_module_file_with_its_library bin \"binary_sharing_a_module_file_with_its_library\" among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate (own code) -> crate::support:\n    \
            crate uses crate::support at src/main.rs:6:5\n  \
            crate::support -> crate (own code):\n    \
            crate::support uses crate at src/support.rs:1:20\n    \
            crate::support uses crate at src/support.rs:2:5\n\
            cycle in binary_sharing_a_module_file_with_its_library lib among the parts of crate (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate (own code) -> crate::support:\n    \
            crate uses crate::support at src/lib.rs:7:9\n  \
            crate::support -> crate (own code):\n    \
            crate::support uses crate at src/support.rs:1:20\n    \
            crate::support uses crate at src/support.rs:2:5\n\
            violation: module file src/support.rs belongs to 2 modules (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu):\n  \
            crate::support in binary_sharing_a_module_file_with_its_library bin \"binary_sharing_a_module_file_with_its_library\"\n  \
            crate::support in binary_sharing_a_module_file_with_its_library lib\n\
            result: 2 cycles, 1 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
