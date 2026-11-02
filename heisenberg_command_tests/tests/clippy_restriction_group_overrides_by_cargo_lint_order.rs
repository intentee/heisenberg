use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn clippy_restriction_group_overrides_by_cargo_lint_order() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_restriction_group",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: ClippyLintNotDenied - overridden/Cargo.toml - Package overridden does not deny clippy::expect_used\n\
            \n\
            ERROR: ClippyLintNotDenied - overridden/Cargo.toml - Package overridden does not deny clippy::panic\n\
            \n\
            ERROR: ClippyLintNotDenied - overridden/Cargo.toml - Package overridden does not deny clippy::unwrap_used\n\
            \n\
            ERROR: ClippyLintNotDenied - same_priority/Cargo.toml - Package same_priority does not deny clippy::unwrap_used\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 6 crate targets, 6 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 6 crate targets, 6 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
