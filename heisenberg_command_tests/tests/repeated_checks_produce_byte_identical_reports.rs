use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn repeated_checks_produce_byte_identical_reports() {
    let workspace = FixtureWorkspace {
        name: "integration_tests_sharing_a_module_file",
    }
    .path();
    let check = HeisenbergCheck {
        workspace: &workspace,
    };
    let first = check.run().expect("the arguments must parse");
    let second = check.run().expect("the arguments must parse");

    assert_eq!(first.output, second.output);
    assert_eq!(first.status, second.status);
}
