use criterion::Criterion;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("construct the library", |bencher| {
        bencher.iter(|| every_target_kind::Library);
    });
    criterion.final_summary();
}
