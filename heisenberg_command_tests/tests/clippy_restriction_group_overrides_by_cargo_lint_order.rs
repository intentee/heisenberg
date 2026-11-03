use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("ClippyLintNotDenied: 4 violations\n\
            Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses; only a package named <something>_tests that no workspace member depends on may leave it undenied, in its own lints table.\n\
            \n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::expect_used\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::panic\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::unwrap_used\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::unwrap_used\n\
            \n\
            LintAllowedInManifest: 4 violations\n\
            Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow; if the lint then reports code, fix that code.\n\
            \n\
            - forbidden/Cargo.toml: Package forbidden allows restriction in its clippy lints\n\
            - overridden/Cargo.toml: Package overridden allows restriction in its clippy lints\n\
            - preceded/Cargo.toml: Package preceded allows restriction in its clippy lints\n\
            - same_priority/Cargo.toml: Package same_priority allows restriction in its clippy lints\n\
            \n\
            ------------------------------\n\
            8 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 6 crate targets, 6 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 6 crate targets, 6 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
