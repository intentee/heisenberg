use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/lib.rs:6:13: Parts of crate depend on each other\n                \
            crate::widths::widths() + crate::OFFSET\n      \
            crate (own code) -> crate::widths:\n        \
            crate uses crate::widths at src/lib.rs:6:13\n      \
            crate::widths -> crate (own code):\n        \
            crate::widths uses crate at src/lib.rs:15:9\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
