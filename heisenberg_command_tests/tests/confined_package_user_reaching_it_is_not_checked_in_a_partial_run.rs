use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn confined_package_user_reaching_it_is_not_checked_in_a_partial_run() {
    let directory = Path::new(env!("CARGO_TARGET_TMPDIR")).join("partial_run");
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: &directory,
            name: "confined_packages",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("ConfinedPackageReachedOutsideItsUsers: 7 violations\n\
            A confined package is built and used only by its declared users, so nothing else reaches what it provides, even through another member. Remove the dependency that the path names; code that needs the package belongs in one of its users.\n\
            \n\
            - both/Cargo.toml: Package both depends on confined package confined without being one of its users\n      \
            both v0.1.0 -> confined v0.1.0\n\
            - builder/Cargo.toml: Package builder depends on confined package confined without being one of its users\n      \
            builder v0.1.0 -> both v0.1.0 -> wrapper v0.1.0 -> confined v0.1.0\n\
            - builder/Cargo.toml: Package builder depends on confined package x without being one of its users\n      \
            builder v0.1.0 -> scripted v0.1.0 -> x v0.1.0\n\
            - enabler/Cargo.toml: Package enabler depends on confined package confined without being one of its users\n      \
            enabler v0.1.0 -> both v0.1.0 -> confined v0.1.0\n\
            - forked/Cargo.toml: Package forked depends on confined package confined without being one of its users\n      \
            forked v0.1.0 -> wrapper v0.1.0 -> confined v0.1.0\n\
            - stranger/Cargo.toml: Package stranger depends on confined package confined without being one of its users\n      \
            stranger v0.1.0 -> user v0.1.0 -> wrapper v0.1.0 -> confined v0.1.0\n\
            - unlisted/Cargo.toml: Package unlisted depends on confined package confined without being one of its users\n      \
            unlisted v0.1.0 -> confined v0.1.0\n\
            \n\
            ConfinedPackageUserListedTwice: 1 violation\n\
            Each user of a confined package is listed once. Remove the repeated entry from confined_packages.\n\
            \n\
            - Cargo.toml: Package user is listed more than once as a user of confined package confined\n\
            \n\
            ConfinedPackageUserNotAMember: 1 violation\n\
            Only a workspace member can use a confined package. Remove the entry from confined_packages, or add the package to the workspace.\n\
            \n\
            - Cargo.toml: Package missing is listed as a user of confined package confined but is not a workspace member\n\
            \n\
            PackageDependencyCycle: 1 violation\n\
            Package dependencies never form a cycle, dev-dependencies included. When a test library depends on the package whose tests use it, move those tests and the helpers that need the package into a separate tests package that holds them in its library.\n\
            \n\
            - cyclic/Cargo.toml: Packages cyclic v0.1.0, harness v0.1.0 depend on each other through dev-dependencies\n\
            \n\
            PathPackageOutsideWorkspace: 8 violations\n\
            Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
            \n\
            - Cargo.toml: Package confined v0.1.0 is built from vendor/confined/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package d v0.1.0 is built from vendor/d/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package p v0.1.0 is built from vendor/p/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package q v0.1.0 is built from vendor/q/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package scripted v0.1.0 is built from vendor/scripted/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package x v0.1.0 is built from vendor/x/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package y v0.1.0 is built from vendor/y/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package z v0.1.0 is built from vendor/z/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            UnknownConfinedPackage: 1 violation\n\
            A confinement holds only for a package that the workspace resolves. Remove the entry from confined_packages, or correct the package name.\n\
            \n\
            - Cargo.toml: Confined package absent names no package that the workspace resolves\n\
            \n\
            ------------------------------\n\
            19 errors found in 6 rules\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}: 19 crate targets, 22 modules\n\
            Not analysed: {HOST_CONFIGURATION}-without-features\n\
            {}", NextestPresenceEvaluation::Evaluated.partial_run_footer_end()).as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
