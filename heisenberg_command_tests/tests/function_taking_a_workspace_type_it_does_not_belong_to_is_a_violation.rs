use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("ProceduralFunction: 4 violations\n\
            Behavior belongs with the data it works on. Make the function a method of the type it takes, a constructor of its own type, or a private helper used in a single file.\n\
            \n\
            - src/describe.rs:4:8: Function takes Report without belonging to it\n        \
            fn describe(report: &Report) -> u8;\n\
            - src/formatter.rs:6:12: Function takes Report without belonging to it\n        \
            pub fn format(report: &Report) -> u8 {\n\
            - src/render.rs:3:8: Function takes Report without belonging to it\n    \
            pub fn render(report: &Report) -> u8 {\n\
            - src/report.rs:3:4: Function takes Report without belonging to it\n    \
            fn line_count(report: &Report) -> u8 {\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 6 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 6 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
