use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn function_like_proc_macro_call_is_compliant() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "function_like_proc_macro_in_expression_position",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            result: 0 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
