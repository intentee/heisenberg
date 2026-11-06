use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_TRIPLE;
use heisenberg_command_tests::constants::LEFT_OUT_TRIPLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 2;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 3;

#[test]
fn code_active_on_no_target_is_not_evaluated_in_a_partial_run() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "code_active_on_no_target",
        }
        .path(),
    }
    .run_with_arguments(&["--target", HOST_TRIPLE])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on {HOST_TRIPLE}: 1 crate targets, {HOST_MODULES} modules\n\
            Not analysed: {LEFT_OUT_TRIPLE}\n\
            Features: all\n\
            Not evaluated in a partial run: CodeActiveOnNoTarget, the empty-library check of TestsOnlyPackageWithoutTestLibraries\n\
            Precondition: rustc 1.99.1 compiles the workspace for every analysed target with the selected features\n").as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
