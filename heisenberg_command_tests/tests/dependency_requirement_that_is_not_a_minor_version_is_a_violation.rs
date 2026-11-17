use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn dependency_requirement_that_is_not_a_minor_version_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dependency_requirements",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnpinnedDependencyRequirement: 5 violations\n\
            A requirement of MAJOR.MINOR accepts the compatible releases that Cargo.lock then fixes, a wider one lets a build move to a release that breaks it, and a narrower one blocks the compatible fixes. Write the requirement as MAJOR.MINOR, and give a git dependency the rev it is built from.\n\
            \n\
            - Cargo.toml: Dependency thiserror requires `2`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version\n\
            - built/Cargo.toml: Dependency thiserror requires `2`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version\n\
            - patched/Cargo.toml: Dependency thiserror requires `2.0.0`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version\n\
            - ranged/Cargo.toml: Dependency thiserror requires `>=2.0`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version\n\
            - targeted/Cargo.toml: Dependency thiserror requires `*`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version\n\
            \n\
            ------------------------------\n\
            5 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(7, 7).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
