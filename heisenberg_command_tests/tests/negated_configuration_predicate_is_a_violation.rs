use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const NOT_WINDOWS_ACTIVE_ON_NO_TARGET: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const DEBUG_ASSERTIONS_FOLD: &str = "      code active on no declared target at src/lib.rs:15:12\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const FEATURE_OFF_WINDOWS_FOLD: &str =
    "      code active on no declared target at src/lib.rs:18:12\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const FEATURE_ON_UNIX_FOLD: &str = "      code active on no declared target at src/lib.rs:21:12\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const ERRORS: usize = 6;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const NOT_WINDOWS_ACTIVE_ON_NO_TARGET: &str = "CodeActiveOnNoTarget: 1 violation\n\
    Code active on no declared target is never built or analysed. Remove it, declare a supported target it is active on, or make it active on a declared target.\n\
    \n\
    - src/lib.rs:32:12: Code is active on no declared target\n        \
    pub fn not_windows() {}\n\
    \n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const DEBUG_ASSERTIONS_FOLD: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const FEATURE_OFF_WINDOWS_FOLD: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const FEATURE_ON_UNIX_FOLD: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const ERRORS: usize = 7;

#[test]
fn negated_configuration_predicate_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "negated_configuration_predicates",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "{NOT_WINDOWS_ACTIVE_ON_NO_TARGET}\
             NegatedConfiguration: 6 violations\n\
             Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.\n\
             \n\
             - src/lib.rs:1:1: Configuration predicate negates a feature, test or debug_assertions\n    \
             #![cfg_attr(not(feature = \"optional\"), allow(dead_code))]\n\
             - src/lib.rs:9:5: Configuration predicate negates a feature, test or debug_assertions\n        \
             #[cfg_attr(not(test), must_use)]\n\
             - src/lib.rs:14:5: Configuration predicate negates a feature, test or debug_assertions\n        \
             #[cfg(any(windows, all(unix, not(debug_assertions))))]\n\
             {DEBUG_ASSERTIONS_FOLD}\
             - src/lib.rs:17:5: Configuration predicate negates a feature, test or debug_assertions\n        \
             #[cfg(any(windows, not(feature = \"optional\")))]\n\
             {FEATURE_OFF_WINDOWS_FOLD}\
             - src/lib.rs:20:5: Configuration predicate negates a feature, test or debug_assertions\n        \
             #[cfg_attr(unix, cfg(not(feature = \"optional\")))]\n\
             {FEATURE_ON_UNIX_FOLD}\
             - src/lib.rs:27:9: Configuration predicate negates a feature, test or debug_assertions\n            \
             #[cfg(not(feature = \"optional\"))]\n      \
             code active on no declared target at src/lib.rs:28:9\n\
             \n\
             ------------------------------\n\
             {ERRORS} errors found\n\
             ------------------------------\n\
             {}",
            full_run_footer(1, 1)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
