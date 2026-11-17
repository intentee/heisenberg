use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unsafe_code_in_a_crate_without_foreign_integration_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unsafe_code_without_foreign_integration",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnsafeCode: 4 violations\n\
            Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it. Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.\n\
            \n\
            - src/trusted.rs:1:5: Unsafe code in a crate that integrates no foreign code\n    \
            pub unsafe trait Trusted {}\n\
            - src/value.rs:9:1: Unsafe code in a crate that integrates no foreign code\n    \
            unsafe impl Trusted for Value {}\n\
            - src/value.rs:19:9: Unsafe code in a crate that integrates no foreign code\n            \
            unsafe { Self::unchecked() }.wrapping_add(self.byte).wrapping_add(OFFSET)\n\
            - src/value.rs:23:15: Unsafe code in a crate that integrates no foreign code\n        \
            pub const unsafe fn unchecked() -> u8 {\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
