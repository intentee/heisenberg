use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn missing_clippy_test_allowance_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_test_allowance_missing",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("ClippyTestAllowanceMissing: 3 violations\n\
            Tests may fail fast with expect, unwrap and panic. Set the allowance to true in the clippy configuration.\n\
            \n\
            - clippy.toml: Clippy configuration does not set allow-panic-in-tests = true\n\
            - clippy.toml: Clippy configuration does not set allow-unwrap-in-tests = true\n\
            - nested/clippy.toml: Clippy configuration does not set allow-expect-in-tests = true\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
