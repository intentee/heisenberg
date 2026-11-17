use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const UNIX_SUPPRESSIONS: &str = "- src/suppressed.rs:1:18: Attribute suppresses lints\n    \
    #[cfg_attr(unix, allow(unused_variables))]\n\
    - src/suppressed.rs:9:3: Attribute suppresses lints\n    \
    #[allow(dead_code)]\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const SUPPRESSIONS: usize = 8;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const ERRORS: usize = 9;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const UNIX_SUPPRESSIONS: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const SUPPRESSIONS: usize = 6;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const ERRORS: usize = 7;

#[test]
fn lint_suppression_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lint_suppressions",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "LintSuppression: {SUPPRESSIONS} violations\n\
             Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.\n\
             \n\
             - src/lib.rs:1:4: Attribute suppresses lints\n    \
             #![allow(unused)]\n\
             {UNIX_SUPPRESSIONS}\
             - src/suppressed.rs:12:3: Attribute suppresses lints\n    \
             #[warn(clippy::unwrap_used)]\n\
             - src/suppressed.rs:15:3: Attribute suppresses lints\n    \
             #[warn(clippy::restriction)]\n\
             - src/suppressed.rs:21:7: Attribute suppresses lints\n        \
             #[allow(unused)]\n\
             - src/suppressed.rs:25:3: Attribute suppresses lints\n    \
             #[allow(dead_code)]\n\
             - src/suppressed.rs:28:3: Attribute suppresses lints\n    \
             #[expect(clippy::expect_used, reason = \"the fixture expects\")]\n\
             \n\
             PathPackageOutsideWorkspace: 1 violation\n\
             Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
             \n\
             - Cargo.toml: Package generator v0.1.0 is built from generator/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
             \n\
             ------------------------------\n\
             {ERRORS} errors found in 2 rules\n\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 2)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
