use std::collections::BTreeMap;
use std::ffi::OsString;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("NonDocComment: 1 violation\n\
            Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an API.\n\
            \n\
            - inherited/items.rs:1:1: Comment is not a doc comment\n    \
            // inherited from the environment\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
