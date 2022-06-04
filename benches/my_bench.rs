use criterion::{criterion_group, criterion_main, Criterion};

fn index_item_decode(c: &mut Criterion) {
    c.bench_function("bench", |b| {
        b.iter(|| {
            benchmark_template::bench_me(100);
        });
    });
}

criterion_group!(benches, index_item_decode);
criterion_main!(benches);
