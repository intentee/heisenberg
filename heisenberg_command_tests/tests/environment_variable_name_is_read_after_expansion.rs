use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn environment_variable_name_is_read_after_expansion() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "expanded_environment_variable_names",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("ComputedEnvironmentVariableName: 1 violation\n\
            Heisenberg accounts for the environment variable a macro call reads only when its argument expands to a string literal. Name the variable with a string literal, or with a macro that expands to one.\n\
            \n\
            - src/constants.rs:9:28: Macro call argument does not expand to a string literal\n    \
            pub const NUMBERED: &str = env!(1);\n\
            \n\
            FailedMacroExpansion: 1 violation\n\
            A macro call that fails to expand produces no code to analyse. Fix the call so it expands.\n\
            \n\
            - src/constants.rs:9:28: Macro expansion failed\n    \
            pub const NUMBERED: &str = env!(1);\n      \
            expansion failing at src/constants.rs:9:33\n\
            \n\
            UndeclaredEnvironmentVariable: 3 violations\n\
            A compile-time environment variable that only the shell sets is an input Heisenberg cannot see, so the analysis would use a wrong value. Read only variables that cargo or a build script sets, declare the variable in the [env] table of the cargo configuration, or read it at run time.\n\
            \n\
            - src/constants.rs:3:28: Macro call reads FRAGMENT_UNDECLARED, which neither cargo, a build script nor the [env] configuration sets\n    \
            pub const FRAGMENT: &str = read_variable!(\"FRAGMENT_UNDECLARED\");\n\
            - src/constants.rs:5:24: Macro call reads HELD_UNDECLARED, which neither cargo, a build script nor the [env] configuration sets\n    \
            pub const HELD: &str = env!(held_name!());\n\
            - src/constants.rs:7:34: Macro call reads NESTED_UNDECLARED, which neither cargo, a build script nor the [env] configuration sets\n    \
            pub const NESTED: &str = concat!(env!(\"NESTED_UNDECLARED\"), \"suffix\");\n\
            \n\
            UnparsableMacroExpansion: 1 violation\n\
            A macro expansion that does not parse cannot be analysed. Fix the macro so its output is valid Rust.\n\
            \n\
            - src/constants.rs:9:28: Macro expansion cannot be parsed\n    \
            pub const NUMBERED: &str = env!(1);\n      \
            in the expansion of env called at src/constants.rs:9:28\n      \
            syntax error at src/constants.rs:9:28: expected expression\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
