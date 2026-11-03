use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn column_counts_characters_not_bytes() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "column_after_multibyte_characters",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("GlobImport: 1 violation\n\
            Glob imports hide where names come from. Import each name explicitly.\n\
            \n\
            - src/lib.rs:1:33: Glob import\n    \
            const LABEL: &str = \"żółw\"; use std::fmt::*;\n\
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
