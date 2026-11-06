use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn crate_root_configured_out_is_reported_at_its_start() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "crate_root_configured_out",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["CodeActiveOnNoTarget: 1 violation\n\
            Code active on no declared target is never built or analysed. Remove it, declare a supported target it is active on, or make it active on a declared target.\n\
            \n\
            - src/lib.rs:1:1: Code is active on no declared target\n    \
            #![cfg(any())]\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", full_run_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
