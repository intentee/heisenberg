use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn environment_value_overrides_a_configured_variable_that_is_not_forced() {
    let workspace = FixtureWorkspace {
        name: "declared_environment_value",
    }
    .path();
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run_in(&BTreeMap::from([(
        OsString::from("GENERATED"),
        workspace.join("inherited").into_os_string(),
    )]))
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["IncludedSourceFile: 1 violation\n\
            An included file is trusted as generated text, so code written by hand escapes the rules about how code is written once it is included. Make the file a module, or have a build script generate it into OUT_DIR.\n\
            \n\
            - src/lib.rs:1:1: Included file inherited/items.rs is not a build script output\n    \
            include!(concat!(env!(\"GENERATED\"), \"/items.rs\"));\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
