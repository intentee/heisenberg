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
        Ok(["CodeActiveInNoConfiguration: 1 violation\n\
            Code active in no declared configuration is never built or analysed. Remove it, declare a configuration it is active in, or make it active in a declared configuration.\n\
            \n\
            - src/lib.rs:2:9: Code is active in no declared configuration\n    \
            pub mod configured {\n\
            \n\
            UnresolvedPath: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/lib.rs:13:17: Path compiles, but rust-analyzer cannot resolve it\n        \
            let _item = configured_item!();\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", full_run_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
