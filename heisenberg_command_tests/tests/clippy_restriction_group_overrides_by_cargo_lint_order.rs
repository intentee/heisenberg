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
            RequiredLintNotDenied: 15 violations\n\
            Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items. Deny the lint in the [lints] or [workspace.lints] table the package uses.\n\
            \n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::arithmetic_side_effects\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::expect_used\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::indexing_slicing\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::panic\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::panic_in_result_fn\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::string_slice\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::todo\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::unimplemented\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::unreachable\n\
            - overridden/Cargo.toml: Package overridden does not deny clippy::unwrap_used\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::string_slice\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::todo\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::unimplemented\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::unreachable\n\
            - same_priority/Cargo.toml: Package same_priority does not deny clippy::unwrap_used\n\
            \n\
            ------------------------------\n\
            19 errors found\n\
            ------------------------------\n", host_footer(6, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
