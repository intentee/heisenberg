use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn dependency_requirement_of_a_root_package_is_reported_once() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dependency_requirement_of_a_root_package",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnpinnedDependencyRequirement: 1 violation\n\
            A requirement of MAJOR.MINOR accepts the compatible releases that Cargo.lock then fixes, a wider one lets a build move to a release that breaks it, and a narrower one blocks the compatible fixes. Write the requirement as MAJOR.MINOR, and give a git dependency the rev it is built from.\n\
            \n\
            - Cargo.toml: Dependency thiserror requires `2`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
