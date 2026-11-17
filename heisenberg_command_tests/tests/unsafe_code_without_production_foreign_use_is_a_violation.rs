use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unsafe_code_without_production_foreign_use_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unsafe_code_without_production_foreign_use",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnsafeCode: 3 violations\n\
            Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it. Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.\n\
            \n\
            - empty_block/src/lib.rs:1:1: Unsafe code in a crate that integrates no foreign code\n    \
            unsafe extern \"C\" {}\n\
            - empty_block/src/lib.rs:5:1: Unsafe code in a crate that integrates no foreign code\n    \
            unsafe impl Send for Handle {}\n\
            - tested/src/lib.rs:3:1: Unsafe code in a crate that integrates no foreign code\n    \
            unsafe impl Send for Shared {}\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(2, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
