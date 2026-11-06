use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn parent_and_child_depending_on_each_other_form_a_cycle() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "parent_and_child_depending_on_each_other",
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
            - src/child.rs:4:24: Parts of crate depend on each other\n        \
            pub fn parent() -> crate::Parent {\n      \
            crate (own code) -> crate::child:\n        \
            crate uses crate::child at src/lib.rs:6:23\n        \
            crate uses crate::child at src/lib.rs:7:9\n      \
            crate::child -> crate (own code):\n        \
            crate::child uses crate at src/child.rs:4:24\n        \
            crate::child uses crate at src/child.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
