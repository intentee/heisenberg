use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_exceptions_of_a_rule() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status =
        HeisenbergCommand::try_parse_from([PROGRAM_NAME, EXPLAIN_SUBCOMMAND, "PositionalFields"])
            .expect("the arguments must parse")
            .run(&BTreeMap::new(), &mut output, &mut errors)
            .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 6.3 PositionalFields: A struct or variant with two or more positional fields\n\
        Section 6, Names, data and functions. Scope: module file. Applies to all code. Evaluated in each analysed configuration.\n\
        \n\
        A struct or enum variant written with two or more positional fields.\n\
        \n\
        Exception: A struct or variant with one positional field, which wraps a single value.\n\
        \n\
        Rationale: A positional field has no name, so every use relies on the order of the fields and nothing says what each one means.\n\
        \n\
        Remedy: Give the fields names.\n\
        \n\
        See also: TupleInSignature\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
