use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: LintSuppression - src/lib.rs:1:4 - Attribute suppresses lints\n\
            #![allow(unused)]\n\
            \n\
            ERROR: LintSuppression - src/suppressed.rs:1:3 - Attribute suppresses lints\n\
            #[allow(dead_code)]\n\
            \n\
            ERROR: LintSuppression - src/suppressed.rs:4:3 - Attribute suppresses lints\n\
            #[expect(clippy::expect_used, reason = \"the fixture expects\")]\n\
            \n\
            ERROR: LintSuppression - src/suppressed.rs:7:18 - Attribute suppresses lints (only in lint_suppressions lib on x86_64-unknown-linux-gnu)\n\
            #[cfg_attr(unix, allow(unused_variables))]\n\
            \n\
            ERROR: LintSuppression - src/suppressed.rs:15:3 - Attribute suppresses lints (only in lint_suppressions lib on x86_64-unknown-linux-gnu)\n\
            #[allow(dead_code)]\n\
            \n\
            ERROR: LintSuppression - src/suppressed.rs:18:3 - Attribute suppresses lints\n\
            #[warn(clippy::unwrap_used)]\n\
            \n\
            ERROR: LintSuppression - src/suppressed.rs:21:3 - Attribute suppresses lints\n\
            #[warn(clippy::restriction)]\n\
            \n\
            ------------------------------\n\
            7 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
