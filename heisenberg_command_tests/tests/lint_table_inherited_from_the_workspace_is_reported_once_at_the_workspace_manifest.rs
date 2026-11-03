use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn lint_table_inherited_from_the_workspace_is_reported_once_at_the_workspace_manifest() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "inherited_workspace_lint_table",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("ClippyLintNotDenied: 1 violation\n\
            Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses.\n\
            \n\
            - Cargo.toml: Workspace does not deny clippy::panic\n\
            \n\
            LintAllowedInManifest: 1 violation\n\
            Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow and fix the code the lint reports.\n\
            \n\
            - Cargo.toml: Workspace allows too_many_lines in its clippy lints\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
