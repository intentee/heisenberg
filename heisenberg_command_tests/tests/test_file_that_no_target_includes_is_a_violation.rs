use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn test_file_that_no_target_includes_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "unreachable_test_file",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UnreachableTestFile: 1 violation\n\
            A test file that no target includes is never compiled, so its tests never run and nothing reports it, which is the risk of declaring integration test targets by hand. Include the file as a module of the integration test target, or remove it.\n\
            \n\
            - tests/forgotten.rs: Test file is a module file of no crate target\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            full_run_footer(2, 4).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
