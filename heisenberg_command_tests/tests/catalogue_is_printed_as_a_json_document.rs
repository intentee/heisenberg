use std::collections::BTreeMap;

use clap::Parser;
use serde_json::Value;
use serde_json::from_slice;
use serde_json::json;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CATALOGUE_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn catalogue_is_printed_as_a_json_document() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status =
        HeisenbergCommand::try_parse_from([PROGRAM_NAME, CATALOGUE_SUBCOMMAND, "--format", "json"])
            .expect("the arguments must parse")
            .run(&BTreeMap::new(), &mut output, &mut errors)
            .expect("the errors of the run must be reported");
    let document: Value = from_slice(&output).expect("the catalogue must be a JSON document");

    assert_eq!(document.pointer("/category"), Some(&json!("mandatory")));
    assert_eq!(document.pointer("/decidability"), Some(&json!("decidable")));
    assert_eq!(document.pointer("/version"), Some(&json!("0.1.0")));
    assert_eq!(
        document.pointer("/rules/0"),
        Some(&json!({
            "applies_to": "all code",
            "definition": "A cfg_select! call, whose unselected arms escape the analysis.",
            "evaluation": "in each analysed configuration",
            "exceptions": [],
            "identifier": "H001",
            "name": "ConfigurationSelectionMacro",
            "rationale": "cfg_select! discards the arms it does not select before they can be analysed.",
            "remedy": "Write each arm as an item under its own #[cfg] attribute.",
            "scope": "module file",
            "section": "Analysable code",
            "section_number": 1,
            "see_also": [],
            "summary": "A cfg_select! call"
        }))
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
