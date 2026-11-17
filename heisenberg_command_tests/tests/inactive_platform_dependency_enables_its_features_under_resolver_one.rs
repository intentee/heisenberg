use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn inactive_platform_dependency_enables_its_features_under_resolver_one() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "inactive_platform_dependency_feature_under_resolver_one",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:1:1: Parts of crate depend on each other\n    \
            shared::link!();\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:1:1\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:4:23\n        \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            PathPackageOutsideWorkspace: 1 violation\n\
            Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
            \n\
            - Cargo.toml: Package shared v0.1.0 is built from shared/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
