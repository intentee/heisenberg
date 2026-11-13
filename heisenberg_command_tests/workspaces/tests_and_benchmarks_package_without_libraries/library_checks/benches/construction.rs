use criterion::Criterion;

use library::Library;

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("construct the library", |bencher| {
        bencher.iter(|| Library);
    });
    criterion.final_summary();
}
