use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn module_analysed_differently_per_platform_is_listed_once() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_analysed_differently_per_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: SharedModuleFile - src/shared.rs - Module file belongs to 3 modules\n  \
            crate::first in module_file_analysed_differently_per_target lib (only on x86_64-pc-windows-msvc)\n  \
            crate::second in module_file_analysed_differently_per_target lib, not analysed on x86_64-pc-windows-msvc: the crate already analyses the file as crate::first\n  \
            crate::third in module_file_analysed_differently_per_target lib, not analysed on x86_64-pc-windows-msvc: the crate already analyses the file as crate::first, not analysed on x86_64-unknown-linux-gnu: the crate already analyses the file as crate::second\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
