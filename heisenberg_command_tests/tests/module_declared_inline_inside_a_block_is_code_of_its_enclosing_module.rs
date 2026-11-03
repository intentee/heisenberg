use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn module_declared_inline_inside_a_block_is_code_of_its_enclosing_module() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_declared_inline_inside_a_block",
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
            - src/lib.rs:5:9: Parts of crate depend on each other\n            \
            crate::OFFSET + 2\n      \
            crate (own code) -> crate::widths:\n        \
            crate uses crate::widths at src/lib.rs:12:13\n      \
            crate::widths -> crate (own code):\n        \
            crate::widths uses crate at src/lib.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
