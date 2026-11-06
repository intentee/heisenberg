use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn paths_written_inside_an_own_macro_are_checked() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "own_macro_with_an_unresolvable_body",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["CodeActiveOnNoTarget: 1 violation\n\
            Code active on no declared target is never built or analysed. Remove it, declare a supported target it is active on, or make it active on a declared target.\n\
            \n\
            - src/lib.rs:2:9: Code is active on no declared target\n    \
            pub mod configured {\n\
            \n\
            UnresolvedPath: 1 violation\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/lib.rs:13:17: Path is unresolved\n        \
            let _item = configured_item!();\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", full_run_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
