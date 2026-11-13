use criterion::Criterion;

fn main() {
    let mut criterion = Criterion.sample_size(10);

    criterion.bench_function("sum", || {});
}
