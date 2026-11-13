use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

fn summation(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("summation");

    group.sample_size(10);
    group.bench_function("sum", |bencher| {
        bencher.iter_with_setup(|| 64_u64, |limit| (0..limit).sum::<u64>());
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(1));
    targets = summation
}
criterion_main!(benches);
