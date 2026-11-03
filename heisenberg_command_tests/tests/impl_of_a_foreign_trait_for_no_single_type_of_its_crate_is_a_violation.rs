use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

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
        Ok("ImplWithoutLocalType: 1 violation\n\
            An impl of another crate's trait belongs beside the one type of this crate it serves. Implement the trait for a struct, enum or union of this crate, or for a foreign type with exactly one of them among the trait's type arguments.\n\
            \n\
            - src/lib.rs:9:30: Impl block belongs to no single type or trait of this crate\n    \
            impl Pair<First, Second> for u32 {}\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
