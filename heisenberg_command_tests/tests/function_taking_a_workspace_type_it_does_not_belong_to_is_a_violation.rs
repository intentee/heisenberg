use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn function_taking_a_workspace_type_it_does_not_belong_to_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "procedural_functions",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ProceduralFunction: 4 violations\n\
            Behavior belongs with the data it works on. Make the function a method of the type it takes, a constructor of its own type, or a private helper used in a single file.\n\
            \n\
            - src/describe.rs:4:8: Function takes Report without belonging to it\n        \
            fn describe(report: &Report) -> u8;\n\
            - src/formatter.rs:6:12: Function takes Report without belonging to it\n        \
            pub fn format(Report { lines }: &Report) -> u8 {\n\
            - src/render.rs:3:8: Function takes Report without belonging to it\n    \
            pub fn render(Report { lines }: &Report) -> u8 {\n\
            - src/report.rs:3:4: Function takes Report without belonging to it\n    \
            fn line_count(Report { lines }: &Report) -> u8 {\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n", host_footer(1, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
