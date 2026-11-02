use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok(
            "ERROR: ImplWithoutLocalType - src/counting.rs:5:40 - Impl block does not implement a struct, enum or union of this crate\n\
            impl<TIterator: Iterator> Counting for TIterator {\n\
            \n\
            ERROR: ImplWithoutLocalType - src/greeting.rs:5:19 - Impl block does not implement a struct, enum or union of this crate\n\
            impl Greeting for String {\n\
            \n\
            ERROR: ImplWithoutLocalType - src/local.rs:11:19 - Impl block does not implement a struct, enum or union of this crate\n\
            impl Greeting for Vec<Local> {\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
