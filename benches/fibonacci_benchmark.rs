use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_fibonacci::*;
use std::collections::HashMap;

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    for i in [20, 21].iter() {
        group.bench_with_input(BenchmarkId::new("Standard", i), i, |b, i| {
            b.iter(|| fib_standard(*i))
        });

        group.bench_with_input(BenchmarkId::new("Recursion", i), i, |b, i| {
            b.iter(|| fib_recursive(*i))
        });

        group.bench_with_input(BenchmarkId::new("Memoization", i), i, |b, i| {
            b.iter(|| {
                let mut memo = HashMap::new();
                fib_memoization(*i, &mut memo);
            })
        });

        group.bench_with_input(BenchmarkId::new("Iterator", i), i, |b, i| {
            b.iter(|| {
                FibIterator::default().nth(*i).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
