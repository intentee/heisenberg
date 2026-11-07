use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

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
        Ok(["LintAllowedInManifest: 4 violations\n\
            Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow; if the lint then reports code, fix that code.\n\
            \n\
            - forbidden/Cargo.toml: Package forbidden allows restriction in its clippy lints\n\
            - overridden/Cargo.toml: Package overridden allows restriction in its clippy lints\n\
            - preceded/Cargo.toml: Package preceded allows restriction in its clippy lints\n\
            - same_priority/Cargo.toml: Package same_priority allows restriction in its clippy lints\n\
            \n\
            RequiredLintNotDenied: 4 violations\n\
            Every warning and every lint of the clippy groups fails the build, and production code reports failures as typed errors instead of panicking. Deny the lint in the [lints] or [workspace.lints] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave the panicking lints undenied, in its own lints table.\n\
            \n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::expect_used\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::panic\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::unwrap_used\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::unwrap_used\n\
            \n\
            ------------------------------\n\
            8 errors found\n\
            ------------------------------\n", host_footer(6, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
