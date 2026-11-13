use criterion::Criterion;

use library_tests::sample_sum;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("sum a sample", |bencher| {
        bencher.iter(sample_sum);
    });
    criterion.final_summary();
}
