use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn item_under_a_proc_macro_attribute_is_analysed_as_written() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "proc_macro_attribute_item_analysed_as_written",
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
            - src/first.rs:2:19: Parts of crate depend on each other\n    \
            pub fn first() -> crate::second::Second {\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:2:19\n        \
            crate::first uses crate::second at src/first.rs:3:5\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
