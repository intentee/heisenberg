use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const NEGATED_PREDICATE: &str = "- src/platform.rs:1:1: Configuration predicate negates a feature, test or debug_assertions\n    \
    #![cfg(any(unix, not(feature = \"optional\")))]\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 2;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const NEGATED_PREDICATE: &str = "- src/lib.rs:1:9: Configuration predicate negates a feature, test or debug_assertions\n    \
    pub mod platform;\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 1;

#[test]
fn negated_predicate_of_a_module_file_active_on_one_platform_is_reported_where_it_is_written() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_with_a_negated_predicate_on_one_platform",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "NegatedConfiguration: 1 violation\n\
             Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.\n\
             \n\
             {NEGATED_PREDICATE}\
             \n\
             ------------------------------\n\
             1 errors found in 1 rule\n\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, HOST_MODULES)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
