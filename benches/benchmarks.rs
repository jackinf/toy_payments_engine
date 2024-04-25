use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use toy_payments_engine::run_transactions_from_file;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("transactions 100mb", |b| {
        let mut input_path = PathBuf::from(file!());
        input_path.pop();
        input_path.push("transactions_100mb.csv");
        b.iter(|| run_transactions_from_file(&input_path))
    });

    c.bench_function("transactions 250", |b| {
        let mut input_path = PathBuf::from(file!());
        input_path.pop();
        input_path.push("transactions_250.csv");
        b.iter(|| run_transactions_from_file(&input_path))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
