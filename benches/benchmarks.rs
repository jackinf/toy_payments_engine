use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use toy_payments_engine::read_transactions_from_file;

fn criterion_benchmark(c: &mut Criterion) {
    let mut input_path = PathBuf::from(file!());
    input_path.pop();
    input_path.push("transactions_250.csv");

    c.bench_function("transactions 250", |b| {
        b.iter(|| read_transactions_from_file(&input_path))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
