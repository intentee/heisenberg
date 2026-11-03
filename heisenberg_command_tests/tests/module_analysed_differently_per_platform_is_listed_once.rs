use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - src/shared.rs: Module file belongs to 3 modules\n      \
            crate::first in module_file_analysed_differently_per_target lib (only on x86_64-pc-windows-msvc)\n      \
            crate::second in module_file_analysed_differently_per_target lib, not analysed on x86_64-pc-windows-msvc: the crate already analyses the file as crate::first\n      \
            crate::third in module_file_analysed_differently_per_target lib, not analysed on x86_64-pc-windows-msvc: the crate already analyses the file as crate::first, not analysed on x86_64-unknown-linux-gnu: the crate already analyses the file as crate::second\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
