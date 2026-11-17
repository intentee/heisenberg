use std::fs;
use std::path::Path;
use std::str;

use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn build_script_output_that_cargo_holds_is_reused_by_the_next_check() {
    let workspace = HostOnlyFixture {
        directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
        name: "build_script_with_an_undeclared_input",
    }
    .path()
    .expect("the host-only fixture must be prepared");
    let undeclared_input = workspace.join("undeclared_input");
    let check = HeisenbergCheck {
        workspace: &workspace,
    };
    let cycle = ["ModuleCycle: 1 violation\n\
        Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
        \n\
        - src/first.rs:5:24: Parts of crate depend on each other\n        \
        pub fn second() -> crate::second::Second {\n      \
        crate::first -> crate::second:\n        \
        crate::first uses crate::second at src/first.rs:5:24\n        \
        crate::first uses crate::second at src/first.rs:6:9\n      \
        crate::second -> crate::first:\n        \
        crate::second uses crate::first at src/second.rs:4:23\n        \
        crate::second uses crate::first at src/second.rs:5:9\n\
        \n\
        ------------------------------\n\
        1 errors found in 1 rule\n\
        ------------------------------\n", full_run_footer(2, 4).as_str()]
    .concat();

    fs::write(&undeclared_input, "").expect("the undeclared input must be written");

    let with_the_input = check.run().expect("the arguments must parse").output;

    fs::remove_file(&undeclared_input).expect("the undeclared input must be removed");

    let without_the_input = check.run().expect("the arguments must parse").output;

    assert_eq!(
        [
            str::from_utf8(&with_the_input),
            str::from_utf8(&without_the_input)
        ],
        [Ok(cycle.as_str()), Ok(cycle.as_str())]
    );
}
