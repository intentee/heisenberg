use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unsafe_code_written_in_workspace_macros_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unsafe_code_written_in_workspace_macros",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnresolvedMethodCall: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the call so that rust-analyzer resolves its method, such as by naming the type of its receiver.\n\
            \n\
            - user/src/absent.rs:6:5: Method call compiles, but rust-analyzer cannot resolve its method\n        \
            absent_method!(0_u8);\n\
            \n\
            UnresolvedPath: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - user/src/absent.rs:5:5: Path into maker compiles, but rust-analyzer cannot resolve it\n        \
            absent_value!();\n\
            \n\
            UnsafeCode: 2 violations\n\
            Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it. Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.\n\
            \n\
            - user/src/bytes.rs:6:23: Unsafe code in a crate that integrates no foreign code\n        \
            let written: u8 = zeroed_byte!();\n\
            - user/src/bytes.rs:7:25: Unsafe code in a crate that integrates no foreign code\n        \
            let generated: u8 = generated_zeroed_byte!();\n\
            \n\
            ------------------------------\n\
            4 errors found in 3 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(3, 8).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
