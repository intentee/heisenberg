use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const WINDOWS_ONLY_CYCLE: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const ERRORS: &str = "No errors found!\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const STATUS: HeisenbergExitStatus = HeisenbergExitStatus::Compliant;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const WINDOWS_ONLY_CYCLE: &str = "ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:3:25: Parts of crate depend on each other\n        \
            windows_only::call!(crate::second::Second::helper);\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:3:25\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:6:9\n\
            \n\
";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const ERRORS: &str = "1 errors found in 1 rule\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const STATUS: HeisenbergExitStatus = HeisenbergExitStatus::Violations;

#[test]
fn windows_only_code_and_dependencies_are_active_for_the_windows_target_only() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "target_gated_cycle",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "{WINDOWS_ONLY_CYCLE}\
             ------------------------------\n\
             {ERRORS}\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::NotEvaluated.host_footer(2, 4)
        )
        .as_str())
    );
    assert_eq!(outcome.status, STATUS);
}
