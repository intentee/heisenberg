use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn tool_attribute_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tool_attributes",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ToolAttribute: 4 violations\n\
            A tool attribute changes how clippy lints or how rustfmt formats the code it is written on, so it relaxes the lints and the formatting that the whole workspace is held to. Remove the attribute and fix the code it excused.\n\
            \n\
            - src/lib.rs:4:7: Attribute of the clippy tool\n        \
            #[clippy::cognitive_complexity = \"100\"]\n\
            - src/lib.rs:7:45: Attribute of the rustfmt tool\n        \
            #[cfg_attr(target_pointer_width = \"64\", rustfmt::skip)]\n\
            - src/lib.rs:10:7: Attribute of the clippy tool\n        \
            #[clippy::msrv = \"1.70.0\"]\n\
            - src/lib.rs:13:7: Attribute of the rustfmt tool\n        \
            #[rustfmt::skip]\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 1)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
