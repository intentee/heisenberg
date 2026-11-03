use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn source_line_of_a_crlf_file_has_no_carriage_return() {
    let workspace = FixtureWorkspace {
        name: "crlf_line_endings",
    }
    .path();
    let items = fs::read_to_string(workspace.join("src").join("items.rs"))
        .expect("the fixture holds src/items.rs");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert!(items.contains("\r\n"));
    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("AdditionalPublicItem: 1 violation\n\
            Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.\n\
            \n\
            - src/items.rs:3:8: Module already has a public item\n    \
            pub fn second() {}\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
