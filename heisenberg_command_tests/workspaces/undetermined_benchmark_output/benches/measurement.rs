use criterion::Bencher;
use criterion::Criterion;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("sum", |bencher| {
        measure(bencher, || (0..64_u64).sum::<u64>());
    });
    criterion.bench_function("pair", |bencher| {
        measure_pair(bencher, || (0..64_u64).sum::<u64>());
    });
    criterion.final_summary();
}

fn measure<TOutput>(bencher: &mut Bencher<'_>, routine: impl FnMut() -> TOutput) {
    bencher.iter(routine);
}

fn measure_pair<TOutput>(bencher: &mut Bencher<'_>, mut routine: impl FnMut() -> TOutput) {
    bencher.iter(|| (64_u64, routine()));
}
