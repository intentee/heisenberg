use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn impl_of_a_foreign_trait_for_no_single_type_of_its_crate_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "foreign_trait_impl_without_a_single_local_type",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ImplWithoutLocalType: 1 violation\n\
            An impl of another crate's trait belongs beside the one type of this crate it serves. Implement the trait for a struct, enum or union of this crate, or for a foreign type with exactly one of them among the trait's type arguments.\n\
            \n\
            - src/lib.rs:9:30: Impl block belongs to no single type or trait of this crate\n    \
            impl Pair<First, Second> for u32 {}\n\
            \n\
            PathPackageOutsideWorkspace: 1 violation\n\
            Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
            \n\
            - Cargo.toml: Package outside v0.1.0 is built from outside/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
