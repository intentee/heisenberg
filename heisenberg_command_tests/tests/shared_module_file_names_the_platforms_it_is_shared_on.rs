use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn shared_module_file_names_the_platforms_it_is_shared_on() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_shared_on_some_platforms",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("SharedModuleFile: 3 violations\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/gated/mod.rs: Module file belongs to 2 modules\n      \
            crate::gated in module_file_shared_on_some_platforms example \"demo\"\n      \
            crate::gated in module_file_shared_on_some_platforms test \"first\"\n\
            - tests/platform/mod.rs: Module file belongs to 2 modules (only on x86_64-pc-windows-msvc)\n      \
            crate::platform in module_file_shared_on_some_platforms example \"demo\"\n      \
            crate::platform in module_file_shared_on_some_platforms test \"first\"\n\
            - tests/support/mod.rs: Module file belongs to 2 modules\n      \
            crate::support in module_file_shared_on_some_platforms example \"demo\"\n      \
            crate::support in module_file_shared_on_some_platforms test \"first\"\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 9 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 6 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
