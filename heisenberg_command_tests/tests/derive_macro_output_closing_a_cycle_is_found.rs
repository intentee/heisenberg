use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn derive_macro_output_closing_a_cycle_is_found() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "derive_macro_output_closing_a_cycle",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemOutOfOrder: 2 violations\n\
            Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names.\n\
            \n\
            - src/lib.rs:9:9: Item belongs before described: module declarations precede private helpers\n    \
            pub mod first;\n\
            - src/lib.rs:10:9: Item belongs before described: module declarations precede private helpers\n    \
            pub mod second;\n\
            \n\
            ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:1:3: Parts of crate depend on each other\n    \
            #[derive(described)]\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:1:3\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:2:19\n        \
            crate::second uses crate::first at src/second.rs:3:9\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
