use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unparsable_macro_expansion_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unparsable_macro_expansion",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnparsableMacroExpansion: 4 violations\n\
            A macro expansion that does not parse cannot be analysed. Fix the macro so its output is valid Rust.\n\
            \n\
            - src/lib.rs:9:1: Macro expansion cannot be parsed\n    \
            unparsable_expansion!();\n      \
            in the expansion of unparsable_expansion called at src/lib.rs:9:1\n      \
            syntax error at src/lib.rs:9:1: expected `;`, `{`, or `(`\n      \
            syntax error at src/lib.rs:9:1: expected a name\n\
            - src/lib.rs:17:1: Macro expansion cannot be parsed\n    \
            unparsable_impl!();\n      \
            in the expansion of unparsable_impl called at src/lib.rs:17:1\n      \
            syntax error at src/lib.rs:17:1: expected `{`\n      \
            syntax error at src/lib.rs:17:1: expected type\n\
            - src/lib.rs:26:12: Macro expansion cannot be parsed\n    \
            pub struct Rewritten;\n      \
            in the expansion of unparsable_attribute called at src/lib.rs:25:3\n      \
            syntax error at src/lib.rs:25:3: expected a block\n      \
            syntax error at src/lib.rs:25:3: expected a name\n      \
            syntax error at src/lib.rs:25:3: expected function arguments\n\
            - src/lib.rs:34:3: Macro expansion cannot be parsed\n    \
            #[derive(unparsable_derive)]\n      \
            in the expansion of unparsable_derive called at src/lib.rs:34:3\n      \
            syntax error at src/lib.rs:34:3: expected a block\n      \
            syntax error at src/lib.rs:34:3: expected a name\n      \
            syntax error at src/lib.rs:34:3: expected function arguments\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
