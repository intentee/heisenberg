use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn negated_predicate_of_a_configured_out_module_file_is_reported_at_its_declaration() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "configured_out_module_file_with_a_negated_predicate",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: CodeActiveOnNoTarget - src/lib.rs:1:9 - Code is active on no supported target\n\
            pub mod gated;\n\
            \n\
            ERROR: NegatedConfiguration - src/lib.rs:1:9 - Configuration predicate negates a feature, test or debug_assertions\n\
            pub mod gated;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
