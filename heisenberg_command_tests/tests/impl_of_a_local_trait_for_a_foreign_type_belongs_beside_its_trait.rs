use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn impl_of_a_local_trait_for_a_foreign_type_belongs_beside_its_trait() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "impl_without_local_type",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("MisplacedForeignTypeImpl: 1 violation\n\
            An impl of a trait of this crate for a type it does not define belongs beside the trait, so every impl has exactly one place. Move the impl into the file that defines the trait.\n\
            \n\
            - src/local.rs:11:19: Impl block of Greeting for a foreign type belongs in src/greeting.rs\n    \
            impl Greeting for Vec<Local> {\n\
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
