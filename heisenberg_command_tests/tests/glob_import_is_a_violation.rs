use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn glob_import_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "glob_import",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "GlobImport: 1 violation\n\
            Glob imports hide where names come from. Import each name explicitly.\n\
            \n\
            - src/lib.rs:3:5: Glob import\n    \
            use inner::*;\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
