use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn windows_only_code_and_dependencies_are_active_for_the_windows_target_only() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "target_gated_cycle",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: ModuleCycle - src/first.rs:3:25 - Parts of crate depend on each other (only in target_gated_cycle lib on x86_64-pc-windows-msvc)\n    \
            windows_only::call!(crate::second::Second::helper);\n  \
            crate::first -> crate::second:\n    \
            crate::first uses crate::second at src/first.rs:3:25\n  \
            crate::second -> crate::first:\n    \
            crate::second uses crate::first at src/second.rs:6:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
