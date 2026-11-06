use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_TRIPLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn build_script_output_with_an_invalid_cfg_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "out_dir_with_an_invalid_cfg",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded for {HOST_TRIPLE}\n\
             error: the build script output cfg `not a cfg` cannot be parsed\n\
             error: failed to parse `not a cfg` as a cfg expression: unexpected content `a cfg` found after cfg expression\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
