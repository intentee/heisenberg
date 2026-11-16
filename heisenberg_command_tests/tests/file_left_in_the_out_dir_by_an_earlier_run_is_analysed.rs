use std::fs;
use std::path::Path;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn file_left_in_the_out_dir_by_an_earlier_run_is_analysed() {
    let workspace = HostOnlyFixture {
        directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
        name: "build_script_leaving_a_file_behind",
    }
    .path()
    .expect("the host-only fixture must be prepared");
    let undeclared_input = workspace.join("undeclared_input");
    let check = HeisenbergCheck {
        workspace: &workspace,
    };

    fs::write(&undeclared_input, "").expect("the undeclared input must be written");

    let with_the_input = check.run().expect("the arguments must parse").status;

    fs::remove_file(&undeclared_input).expect("the undeclared input must be removed");

    let without_the_input = check.run().expect("the arguments must parse").status;

    assert_eq!(
        [with_the_input, without_the_input],
        [
            HeisenbergExitStatus::Compliant,
            HeisenbergExitStatus::Compliant
        ]
    );
}
