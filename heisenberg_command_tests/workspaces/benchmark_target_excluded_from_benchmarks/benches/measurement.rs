use criterion::Criterion;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("sum", |bencher| {
        bencher.iter(|| (0..64_u64).sum::<u64>());
    });
    criterion.final_summary();
}
