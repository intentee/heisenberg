use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::COVERAGE_EXPORT_FILE;
use heisenberg_command_tests::coverage_footer::coverage_footer;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_coverage_check::HeisenbergCoverageCheck;

#[test]
fn unexecuted_code_outside_every_function_is_uncovered_code() {
    let fixture = FixtureWorkspace {
        name: "coverage_code_outside_functions",
    };
    let outcome = HeisenbergCoverageCheck {
        export: &fixture.coverage_export(COVERAGE_EXPORT_FILE),
        workspace: &fixture.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UncoveredCode: 2 violations\n\
            Every line of production code runs under the tests. Add tests that execute the code, or remove the code that nothing needs.\n\
            \n\
            - src/lib.rs:1:12: No test executes parts of the item\n    \
            pub static DOUBLE: fn(u32) -> u32 = |value| value * 2;\n      \
            not executed at src/lib.rs:1:45\n      \
            not executed at src/lib.rs:1:53\n\
            - src/lib.rs:3:12: No test executes parts of the item\n    \
            pub static PARITY: fn(u32) -> &'static str = |value| if value % 2 == 0 { \"even\" } else { \"odd\" };\n      \
            not executed at src/lib.rs:3:90\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n",
            coverage_footer(1, 1, 9).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
