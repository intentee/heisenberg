use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn code_active_on_no_supported_target_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "code_active_on_no_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("CodeActiveOnNoTarget: 6 violations\n\
            Code active on no declared target is never built or analysed. Remove it, declare a supported target it is active on, or make it active on a declared target.\n\
            \n\
            - src/lib.rs:2:9: Code is active on no declared target\n    \
            pub mod mac;\n\
            - src/lib.rs:6:9: Code is active on no declared target\n    \
            pub mod configured_out;\n\
            - src/never.rs:5:12: Code is active on no declared target\n        \
            pub fn unreachable() {}\n\
            - src/never.rs:9:9: Code is active on no declared target\n            \
            return;\n\
            - src/never.rs:16:16: Code is active on no declared target\n            \
            pub fn generated_never() {}\n\
            - src/windows_only.rs:5:12: Code is active on no declared target\n        \
            pub fn unix_inside_windows() {}\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
