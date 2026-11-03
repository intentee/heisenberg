use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn path_resolved_inside_a_dependency_macro_closes_a_cycle_at_the_call() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dependency_macro_referencing_the_calling_crate",
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
            - src/caller.rs:2:5: Parts of crate depend on each other\n        \
            calling::call_home!();\n      \
            crate::caller -> crate::home:\n        \
            crate::caller uses crate::home at src/caller.rs:2:5\n      \
            crate::home -> crate::caller:\n        \
            crate::home uses crate::caller at src/home.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
