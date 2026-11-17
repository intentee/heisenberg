use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_required_lints() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "RequiredLintNotDenied",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 3.6 RequiredLintNotDenied: A workspace member that leaves a required lint or lint group below deny\n\
        Section 3, Lints and formatting. Scope: package. Applies to all code. Evaluated in each analysed configuration.\n\
        \n\
        A workspace member whose lints, after workspace inheritance and in cargo's order, leave a required lint or lint group below deny, under either spelling of its name or a name clippy renamed to it, a warn level counting as deny while the warnings group is denied. Every member, a test crate included, denies the warnings group, the clippy all, nursery and pedantic groups and the panicking lints, the clippy lints that report code that can panic. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].\n\
        \n\
        Required lint groups: clippy::all, clippy::nursery, clippy::pedantic, warnings\n\
        \n\
        Panicking lints: clippy::arithmetic_side_effects, clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::panic_in_result_fn, clippy::string_slice, clippy::todo, clippy::unimplemented, clippy::unreachable, clippy::unwrap_used\n\
        \n\
        Rationale: Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items.\n\
        \n\
        Remedy: Deny the lint in the [lints] or [workspace.lints] table the package uses.\n\
        \n\
        See also: LintAllowedInManifest, LintSuppression, ToolAttribute\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
