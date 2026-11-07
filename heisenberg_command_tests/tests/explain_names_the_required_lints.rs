use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

#[test]
fn explain_names_the_required_lints() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status =
        HeisenbergCommand::try_parse_from(["heisenberg", "explain", "RequiredLintNotDenied"])
            .expect("the arguments must parse")
            .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "RequiredLintNotDenied: A workspace member that leaves a required lint or lint group below deny\n\
        \n\
        A workspace member whose lints, after workspace inheritance and in cargo's order, leave a required lint or lint group below deny, under either spelling of its name or a name clippy renamed to it, a warn level counting as deny while the warnings group is denied. Every member denies the warnings group and the clippy all, nursery and pedantic groups; every member but a test crate also denies the panicking lints, while a test crate, which may panic, declares its own [lints] table, since cargo cannot override single lints of an inherited table. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].\n\
        \n\
        Required lint groups: clippy::all, clippy::nursery, clippy::pedantic, warnings\n\
        \n\
        Panicking lints: clippy::expect_used, clippy::panic, clippy::unwrap_used\n\
        \n\
        Every warning and every lint of the clippy groups fails the build, and production code reports failures as typed errors instead of panicking. Deny the lint in the [lints] or [workspace.lints] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave the panicking lints undenied, in its own lints table.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
