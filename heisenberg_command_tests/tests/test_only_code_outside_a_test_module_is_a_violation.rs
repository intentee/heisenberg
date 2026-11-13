use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn test_only_code_outside_a_test_module_is_a_violation() {
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
        Ok(["TestOnlyCodeInProduction: 4 violations\n\
            Production code holds only what production runs. Keep test-only code in a #[cfg(test)] module, such as the inline mod tests of the file it tests, and move test libraries shared by several packages into a test crate.\n\
            \n\
            - src/counter.rs:2:10: Code exists only under cfg(test) outside a test module\n    \
            const fn module_helper() -> u8 {\n\
            - src/counter.rs:12:18: Code exists only under cfg(test) outside a test module\n        \
            pub const fn for_tests() -> Self {\n\
            - src/counter.rs:18:9: Code exists only under cfg(test) outside a test module\n            \
            assert!(self.value < u8::MAX);\n\
            - src/counter.rs:25:6: Code exists only under cfg(test) outside a test module\n    \
            impl Counter {\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(1, 8).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
