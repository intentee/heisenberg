use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn negative_literal_from_a_procedural_macro_is_named_with_its_fix() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "negative_literal_through_a_declarative_macro",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["PathPackageOutsideWorkspace: 1 violation\n\
            Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
            \n\
            - Cargo.toml: Package literal_emitter v0.1.0 is built from literal_emitter/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            UnparsableMacroExpansion: 1 violation\n\
            A macro expansion that does not parse cannot be analysed. Fix the macro so its output is valid Rust.\n\
            \n\
            - src/lib.rs:7:3: Macro expansion cannot be parsed\n    \
            #[literal_emitter::single_token]\n      \
            in the expansion of single_token called at src/lib.rs:7:3\n      \
            in the expansion of wrap called at literal_emitter/src/lib.rs:31:8\n      \
            syntax error at src/lib.rs:7:3: expected expression\n      \
            literal -1i32 at literal_emitter/src/lib.rs:31:8 cannot be read back by rust-analyzer, as a negative number that a procedural macro emits as one literal token cannot; emit the sign as a separate - token\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
