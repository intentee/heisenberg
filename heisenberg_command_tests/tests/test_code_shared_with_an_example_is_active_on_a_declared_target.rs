use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn test_code_shared_with_an_example_is_active_on_a_declared_target() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "test_code_shared_with_an_example",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "SharedModuleFile: 1 violation\n\
             A file included as several modules has no single owner. Give each module its own file.\n\
             \n\
             - tests/support/mod.rs: Module file belongs to 2 modules\n      \
             crate::support in test_code_shared_with_an_example example \"demo\"\n      \
             crate::support in test_code_shared_with_an_example test \"first\"\n\
             \n\
             ------------------------------\n\
             1 errors found\n\
             ------------------------------\n",
            full_run_footer(3, 5).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
