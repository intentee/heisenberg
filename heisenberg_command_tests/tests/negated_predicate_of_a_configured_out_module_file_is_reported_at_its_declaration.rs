use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn negated_predicate_of_a_configured_out_module_file_is_reported_at_its_declaration() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "configured_out_module_file_with_a_negated_predicate",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["NegatedConfiguration: 1 violation\n\
            Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.\n\
            \n\
            - src/lib.rs:1:9: Configuration predicate negates a feature, test or debug_assertions\n    \
            pub mod gated;\n      \
            code active on no declared target at src/lib.rs:1:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", full_run_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
