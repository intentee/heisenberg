use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn extern_crate_declaration_is_not_a_reference_to_a_function_of_its_name() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "function_named_like_an_extern_crate",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "ItemUsedOnlyByTests: 1 violation\n\
            Production code holds only what production uses. Move an item that only tests use into the test module that uses it, or into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.\n\
            \n\
            - src/lib.rs:3:4: Function is used only by tests\n    \
            fn alloc() -> usize {\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated
                .host_footer(1, 2)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
