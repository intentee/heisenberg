use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_SELECTION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn build_script_sees_the_job_count_of_the_check() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "job_count_seen_by_its_build_script",
        }
        .path(),
    }
    .run_with_arguments(&[HOST_SELECTION.as_slice(), &["--jobs", "3"]].concat())
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:5:24: Parts of crate depend on each other\n        \
            pub fn second() -> crate::second::Second {{\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:5:24\n        \
            crate::first uses crate::second at src/first.rs:6:9\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:4:23\n        \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            {footer}", footer = host_footer(2, 4)).as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
