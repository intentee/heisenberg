use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn impl_of_a_local_trait_object_belongs_beside_its_trait() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "trait_object_impls",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("MisplacedTraitObjectImpl: 1 violation\n\
            A trait object's behavior belongs beside its trait. Move the impl into the file that defines the trait.\n\
            \n\
            - src/process_description.rs:3:6: Impl block on a trait object belongs in src/managed_process.rs\n    \
            impl dyn ManagedProcess {\n\
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
