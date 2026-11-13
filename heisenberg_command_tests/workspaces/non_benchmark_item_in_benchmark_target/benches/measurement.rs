use criterion::Criterion;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("sum", |bencher| {
        bencher.iter(|| (0..sample_size()).sum::<u64>());
    });
    criterion.final_summary();
}

pub const fn sample_size() -> u64 {
    64
}
