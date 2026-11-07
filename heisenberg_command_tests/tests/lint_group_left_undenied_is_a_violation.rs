use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn lint_group_left_undenied_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lint_groups_not_denied",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RequiredLintNotDenied: 4 violations\n\
            Every warning and every lint of the clippy groups fails the build, and production code reports failures as typed errors instead of panicking. Deny the lint in the [lints] or [workspace.lints] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave the panicking lints undenied, in its own lints table.\n\
            \n\
            - Cargo.toml: Package lint_groups_not_denied does not deny clippy::all\n\
            - Cargo.toml: Package lint_groups_not_denied does not deny clippy::nursery\n\
            - Cargo.toml: Package lint_groups_not_denied does not deny clippy::pedantic\n\
            - Cargo.toml: Package lint_groups_not_denied does not deny warnings\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
