use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn source_file_that_no_target_includes_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "unreachable_source_files",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnreachableSourceFile: 3 violations\n\
            A source file that no target includes is never compiled or analysed, so its code and its tests never run and nothing reports it. Declare the file as a module of a crate target, or remove it.\n\
            \n\
            - scripts/stray.rs: Source file is a module file of no crate target\n\
            - src/forgotten.rs: Source file is a module file of no crate target\n\
            - tests/forgotten.rs: Source file is a module file of no crate target\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n",
            full_run_footer(2, 4).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
