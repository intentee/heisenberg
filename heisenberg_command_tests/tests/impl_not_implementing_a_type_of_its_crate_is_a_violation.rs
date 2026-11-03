use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn impl_not_implementing_a_type_of_its_crate_is_a_violation() {
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
        Ok("ImplWithoutLocalType: 2 violations\n\
            Behavior belongs with the type or trait that owns it, so impls do not extend foreign types. Implement it for a type of this crate, convert from a type of this crate, or write a blanket or trait-object impl beside its trait.\n\
            \n\
            - src/greeting.rs:5:19: Impl block belongs to no type or trait of this crate\n    \
            impl Greeting for String {\n\
            - src/local.rs:11:19: Impl block belongs to no type or trait of this crate\n    \
            impl Greeting for Vec<Local> {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
