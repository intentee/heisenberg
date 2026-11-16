use std::fmt::Display;
use std::iter;
use std::num::ParseIntError;
use std::pin::pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use criterion::BatchSize;
use criterion::Bencher;
use criterion::Criterion;
use criterion::async_executor::AsyncExecutor;

fn label() -> impl Display {
    64_u64
}

fn numbers() -> impl Iterator<Item = u64> {
    iter::empty()
}

fn parsed() -> Result<u64, ParseIntError> {
    "64".parse()
}

struct ImmediateExecutor;

impl AsyncExecutor for ImmediateExecutor {
    fn block_on<TOutput>(&self, future: impl Future<Output = TOutput>) -> TOutput {
        let mut pinned = pin!(future);
        let mut context = Context::from_waker(Waker::noop());

        loop {
            if let Poll::Ready(output) = pinned.as_mut().poll(&mut context) {
                return output;
            }
        }
    }
}

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    criterion.bench_function("parse", |bencher| {
        bencher.iter(parsed);
    });
    criterion.bench_function("map lazily", |bencher| {
        bencher.iter(|| iter::repeat_n(1_u64, 64).map(u64::count_ones));
    });
    criterion.bench_function("build a closure", |bencher| {
        bencher.iter(|| || 64_u64);
    });
    criterion.bench_function("parse through a path", |bencher| {
        Bencher::iter(bencher, parsed);
    });
    criterion.bench_function("parse a batch", |bencher| {
        bencher.iter_batched(|| "64", |input| Box::new(input.parse::<u64>()), BatchSize::SmallInput);
    });
    criterion.bench_function("parse into a tuple", |bencher| {
        bencher.iter(|| (64_u64, parsed()));
    });
    criterion.bench_function("parse into an array", |bencher| {
        bencher.iter(|| [parsed()]);
    });
    criterion.bench_function("iterate a trait object", |bencher| {
        bencher.iter(|| -> Box<dyn Iterator<Item = u64>> { Box::new(iter::empty()) });
    });
    criterion.bench_function("iterate an opaque type", |bencher| {
        bencher.iter(numbers);
    });
    criterion.bench_function("parse asynchronously", |bencher| {
        bencher.to_async(ImmediateExecutor).iter(|| async { parsed() });
    });
    criterion.bench_function("label", |bencher| {
        bencher.iter(label);
    });
    criterion.bench_function("count", |bencher| {
        bencher.iter(|| (64_u64, [(); 0]));
    });
    criterion.final_summary();
}
