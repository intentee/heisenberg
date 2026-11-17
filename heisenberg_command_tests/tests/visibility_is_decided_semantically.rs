use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn visibility_is_decided_semantically() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "visibility_decided_semantically",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["AdditionalPublicItem: 1 violation\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - src/items.rs:9:15: Module already has a public item\n    \
            pub(super) fn sibling() {}\n\
            \n\
            PublicConstantOutsideConstantsModule: 1 violation\n\
            Public constants have a single home, crate::constants. Move the constant there.\n\
            \n\
            - src/hidden.rs:3:11: Public constant belongs in crate::constants\n    \
            pub const HIDDEN: u8 = 2;\n\
            \n\
            Reexport: 1 violation\n\
            Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it.\n\
            \n\
            - src/items.rs:3:16: Re-export\n    \
            pub(crate) use std::fmt::Write;\n\
            \n\
            ------------------------------\n\
            3 errors found in 3 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
