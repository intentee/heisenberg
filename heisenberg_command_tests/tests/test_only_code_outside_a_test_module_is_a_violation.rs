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
        Ok(["TestOnlyCodeInProduction: 13 violations\n\
            Production code holds only what production runs, and behaves the same whether or not it is under test. Keep test-only code in the inline #[cfg(test)] mod tests of the file it tests, and move test libraries shared by several packages into a test crate.\n\
            \n\
            - src/counter.rs:2:5: Code depends on cfg(test) outside a test module\n    \
            use std::mem;\n\
            - src/counter.rs:5:10: Code depends on cfg(test) outside a test module\n    \
            const fn module_helper() -> u8 {\n\
            - src/counter.rs:15:18: Code depends on cfg(test) outside a test module\n        \
            pub const fn for_tests() -> Self {\n\
            - src/counter.rs:20:18: Code depends on cfg(test) outside a test module\n        \
            pub const fn attributed(&self) -> u8 {\n\
            - src/counter.rs:26:9: Code depends on cfg(test) outside a test module\n            \
            assert!(self.value < u8::MAX);\n\
            - src/counter.rs:32:18: Code depends on cfg(test) outside a test module\n        \
            pub const fn probed(&self) -> u8 {\n\
            - src/counter.rs:37:9: Code depends on cfg(test) outside a test module\n            \
            cfg!(any(all(feature = \"probe\", not(test)))) && self.value == 0\n\
            - src/counter.rs:42:6: Code depends on cfg(test) outside a test module\n    \
            impl Counter {\n\
            - src/probe.rs:4:9: Code depends on cfg(test) outside a test module\n        \
            pub observed: u8,\n\
            - src/probe.rs:11:9: Code depends on cfg(test) outside a test module\n            \
            let _observed = self.observed;\n\
            - src/probe.rs:15:13: Code depends on cfg(test) outside a test module\n                \
            0 => 0,\n\
            - src/step.rs:4:5: Code depends on cfg(test) outside a test module\n        \
            Probed,\n\
            - src/step.rs:8:5: Code depends on cfg(test) outside a test module\n    \
            mod tests {\n\
            \n\
            ------------------------------\n\
            13 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(1, 8).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
