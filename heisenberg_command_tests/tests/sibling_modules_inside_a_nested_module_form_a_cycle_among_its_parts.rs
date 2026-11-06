use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn sibling_modules_inside_a_nested_module_form_a_cycle_among_its_parts() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "siblings_inside_a_nested_module",
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
            - src/outer/first.rs:4:24: Parts of crate::outer depend on each other\n        \
            pub fn second() -> super::second::Second {\n      \
            crate::outer::first -> crate::outer::second:\n        \
            crate::outer::first uses crate::outer::second at src/outer/first.rs:4:24\n        \
            crate::outer::first uses crate::outer::second at src/outer/first.rs:5:9\n      \
            crate::outer::second -> crate::outer::first:\n        \
            crate::outer::second uses crate::outer::first at src/outer/second.rs:4:23\n        \
            crate::outer::second uses crate::outer::first at src/outer/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
