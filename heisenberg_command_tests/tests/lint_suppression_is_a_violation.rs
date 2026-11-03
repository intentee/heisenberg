use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("LintSuppression: 8 violations\n\
            Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.\n\
            \n\
            - src/lib.rs:1:4: Attribute suppresses lints\n    \
            #![allow(unused)]\n\
            - src/suppressed.rs:1:3: Attribute suppresses lints\n    \
            #[allow(dead_code)]\n\
            - src/suppressed.rs:4:3: Attribute suppresses lints\n    \
            #[expect(clippy::expect_used, reason = \"the fixture expects\")]\n\
            - src/suppressed.rs:7:18: Attribute suppresses lints (only in lint_suppressions lib on x86_64-unknown-linux-gnu)\n    \
            #[cfg_attr(unix, allow(unused_variables))]\n\
            - src/suppressed.rs:15:3: Attribute suppresses lints (only in lint_suppressions lib on x86_64-unknown-linux-gnu)\n    \
            #[allow(dead_code)]\n\
            - src/suppressed.rs:18:3: Attribute suppresses lints\n    \
            #[warn(clippy::unwrap_used)]\n\
            - src/suppressed.rs:21:3: Attribute suppresses lints\n    \
            #[warn(clippy::restriction)]\n\
            - src/suppressed.rs:27:7: Attribute suppresses lints\n        \
            #[allow(unused)]\n\
            \n\
            ------------------------------\n\
            8 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
