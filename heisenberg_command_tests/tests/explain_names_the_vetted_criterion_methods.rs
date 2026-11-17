use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_vetted_criterion_methods() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "UnvettedCriterionMethod",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 9.10 UnvettedCriterionMethod: A criterion method that Heisenberg has not vetted\n\
        Section 9, Benchmarks. Scope: module file. Evaluated in each analysed configuration.\n\
        \n\
        A call or reference, in the code of a workspace member, to an inherent method of criterion's Criterion, BenchmarkGroup, Bencher or AsyncBencher that Heisenberg has not vetted: one that changes how long or how many samples it measures, how it judges results or which benchmarks run, a method that criterion hides from its documentation, or a method that Heisenberg does not know. criterion is the criterion package from crates.io, and the methods of traits that these types implement are not inherent methods.\n\
        \n\
        Vetted methods: AsyncBencher::{iter, iter_batched, iter_batched_ref, iter_custom, iter_with_large_drop}, Bencher::{iter, iter_batched, iter_batched_ref, iter_custom, iter_with_large_drop, to_async}, BenchmarkGroup::{bench_function, bench_with_input, finish, plot_config, throughput}, Criterion::{bench_function, bench_with_input, benchmark_group, configure_from_args, final_summary, output_directory, plotting_backend, retain_baseline, save_baseline, with_measurement, with_output_color, with_plots, with_profiler, without_plots}\n\
        \n\
        Rationale: Every benchmark is measured with criterion's defaults, the same way every time.\n\
        \n\
        Remedy: Remove the call, and make the benchmark that needed it faster instead of shortening, weakening or skipping its measurement.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
