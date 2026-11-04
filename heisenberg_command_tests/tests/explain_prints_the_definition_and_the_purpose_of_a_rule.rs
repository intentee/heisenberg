use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

#[test]
fn explain_prints_the_definition_and_the_purpose_of_a_rule() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from(["heisenberg", "explain", "GlobImport"])
        .expect("the arguments must parse")
        .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok("GlobImport: A glob import\n\
        \n\
        A glob import.\n\
        \n\
        Glob imports hide where names come from. Import each name explicitly.\n\
        ")
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
