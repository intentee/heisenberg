use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unvetted_criterion_method_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unvetted_criterion_method",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnvettedCriterionMethod: 3 violations\n\
            Every benchmark is measured with criterion's defaults, the same way every time. Remove the call, and make the benchmark that needed it faster instead of shortening, weakening or skipping its measurement.\n\
            \n\
            - benches/measurement.rs:10:11: criterion method BenchmarkGroup::sample_size is not vetted\n        \
            group.sample_size(10);\n\
            - benches/measurement.rs:12:17: criterion method Bencher::iter_with_setup is not vetted\n            \
            bencher.iter_with_setup(|| 64_u64, |limit| (0..limit).sum::<u64>());\n\
            - benches/measurement.rs:19:35: criterion method Criterion::measurement_time is not vetted\n        \
            config = Criterion::default().measurement_time(Duration::from_secs(1));\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
