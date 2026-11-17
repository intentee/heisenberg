use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unsafe_code_without_the_unsafe_keyword_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unsafe_code_without_the_unsafe_keyword",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnsafeCode: 5 violations\n\
            Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it. Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.\n\
            \n\
            - src/assembly.rs:3:1: Unsafe code in a crate that integrates no foreign code\n    \
            global_asm!(\"nop\");\n\
            - src/exported.rs:1:3: Unsafe code in a crate that integrates no foreign code\n    \
            #[no_mangle]\n\
            - src/marked.rs:1:3: Unsafe code in a crate that integrates no foreign code\n    \
            #[unsafe(no_mangle)]\n\
            - src/named.rs:1:3: Unsafe code in a crate that integrates no foreign code\n    \
            #[export_name = \"renamed\"]\n\
            - src/placed.rs:1:3: Unsafe code in a crate that integrates no foreign code\n    \
            #[link_section = \".placed\"]\n\
            \n\
            ------------------------------\n\
            5 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
