use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn import_disabled_through_cfg_attr_creates_no_module_dependency() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cfg_attr_disabled_import_of_a_sibling",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            result: 0 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
