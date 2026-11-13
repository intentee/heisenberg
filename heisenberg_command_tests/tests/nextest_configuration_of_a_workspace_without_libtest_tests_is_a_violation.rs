use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn nextest_configuration_of_a_workspace_without_libtest_tests_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "unneeded_nextest_configuration",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UnneededNextestConfiguration: 1 violation\n\
            Nextest runs only the tests that libtest's harness defines, so a workspace whose tests all bring their own harness never uses its nextest configuration, and a configuration that nothing reads only suggests limits that nothing enforces. Remove the configuration.\n\
            \n\
            - .config/nextest.toml: Nextest configuration of a workspace that runs no test under libtest's harness\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            full_run_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
