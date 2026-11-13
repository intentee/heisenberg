use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const LEFT_OUT_CONFIGURATIONS: &str =
    "x86_64-pc-windows-msvc, x86_64-pc-windows-msvc-without-features, x86_64-unknown-linux-gnu";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const LEFT_OUT_CONFIGURATIONS: &str =
    "x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu, x86_64-unknown-linux-gnu-without-features";

#[test]
fn code_behind_an_unselected_feature_is_not_analysed() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "feature_gated_cycle",
        }
        .path(),
    }
    .run_with_arguments(&[&format!(
        "--configuration={HOST_CONFIGURATION}-without-features"
    )])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}-without-features: 1 crate targets, 3 modules\n\
            Not analysed: {LEFT_OUT_CONFIGURATIONS}\n\
            {}",
            NextestPresenceEvaluation::NotEvaluated.partial_run_footer_end()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
