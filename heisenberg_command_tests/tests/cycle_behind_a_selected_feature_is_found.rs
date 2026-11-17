use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const LEFT_OUT_CONFIGURATIONS: &str = "x86_64-pc-windows-msvc, x86_64-pc-windows-msvc-without-features, x86_64-unknown-linux-gnu-without-features";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const LEFT_OUT_CONFIGURATIONS: &str = "x86_64-pc-windows-msvc-without-features, x86_64-unknown-linux-gnu, x86_64-unknown-linux-gnu-without-features";

#[test]
fn cycle_behind_a_selected_feature_is_found() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "feature_gated_cycle",
        }
        .path(),
    }
    .run_with_arguments(&[CONFIGURATION_ARGUMENT, HOST_CONFIGURATION])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:5:24: Parts of crate depend on each other\n        \
            pub fn second() -> crate::second::Second {{\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:5:24\n        \
            crate::first uses crate::second at src/first.rs:6:9\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:4:23\n        \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}: 1 crate targets, 3 modules\n\
            Not analysed: {LEFT_OUT_CONFIGURATIONS}\n\
            {}", NextestPresenceEvaluation::NotEvaluated.partial_run_footer_end()).as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
