use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn workspace_type_of_another_member_is_recognised_in_a_crate_sharing_a_module_file() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "workspace_type_in_a_crate_sharing_a_module_file",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ResultErrorNotAnError: 1 violation\n\
            Err is reserved for failures. A value that is not an error type is either an outcome of the regular flow handed back through Err, which hides an expected result where failures belong, or a failure without the contract of an error, its message and its source chain. When the value is a failure, make it the error type of its crate, or a variant of it, deriving thiserror::Error. When it is an outcome of the regular flow, return it in Ok, as a variant of an enum of outcomes.\n\
            \n\
            - sharing/src/parsed.rs:3:30: Result error type Rejection is a type of this workspace that is not an error\n    \
            pub fn parsed(text: &str) -> Result<usize, Rejection> {\n\
            \n\
            SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - sharing/src/shared.rs: Module file belongs to 2 modules\n      \
            crate::shared in sharing bin \"sharing\"\n      \
            crate::shared in sharing lib\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(3, 7).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
