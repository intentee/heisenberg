use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn test_only_code_outside_the_unit_test_module_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "test_only_code_in_production",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("TestOnlyCodeInProduction: 6 violations\n\
            Production code holds only what production runs. Keep unit tests in the inline #[cfg(test)] mod tests of the file they test, and move every other piece of test-only code into a test crate.\n\
            \n\
            - src/counter.rs:8:9: Code exists only under cfg(test) outside the unit-test module\n            \
            assert!(self.value < u8::MAX);\n\
            - src/counter.rs:14:18: Code exists only under cfg(test) outside the unit-test module\n        \
            pub const fn for_tests() -> Self {\n\
            - src/counter.rs:20:6: Code exists only under cfg(test) outside the unit-test module\n    \
            impl Counter {\n\
            - src/counter.rs:27:10: Code exists only under cfg(test) outside the unit-test module\n    \
            const fn module_helper() -> u8 {\n\
            - src/counter.rs:32:5: Code exists only under cfg(test) outside the unit-test module\n    \
            mod checks {\n\
            - src/lib.rs:5:5: Code exists only under cfg(test) outside the unit-test module\n    \
            mod tests;\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 6 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 6 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
