use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn inline_assembly_bodies_are_compared_by_their_operands() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "asm_template_operands",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "DuplicatedFunctionBody: 1 violation\n\
            Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.\n\
            \n\
            - src/first.rs:3:8: Function body is duplicated\n    \
            pub fn first(value: u64) -> u64 {\n      \
            duplicate at src/second.rs:3:8\n\
            \n\
            UnsafeCode: 2 violations\n\
            Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it. Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.\n\
            \n\
            - src/first.rs:6:5: Unsafe code in a crate that integrates no foreign code\n        \
            unsafe {\n\
            - src/second.rs:6:5: Unsafe code in a crate that integrates no foreign code\n        \
            unsafe {\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
