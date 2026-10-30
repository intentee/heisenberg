use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: ModuleCycle - src/outer/first.rs:4:24 - Parts of crate::outer depend on each other\n    \
            pub fn second() -> super::second::Second {\n  \
            crate::outer::first -> crate::outer::second:\n    \
            crate::outer::first uses crate::outer::second at src/outer/first.rs:4:24\n    \
            crate::outer::first uses crate::outer::second at src/outer/first.rs:5:9\n  \
            crate::outer::second -> crate::outer::first:\n    \
            crate::outer::second uses crate::outer::first at src/outer/second.rs:4:23\n    \
            crate::outer::second uses crate::outer::first at src/outer/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
