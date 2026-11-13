use criterion::Criterion;

use function_used_only_by_benchmarks::sum;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("sum", |bencher| {
        bencher.iter(|| sum(64));
    });
    criterion.final_summary();
}
