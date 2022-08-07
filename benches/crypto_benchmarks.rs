use criterion::{black_box, Criterion, criterion_group, criterion_main};
use karkinos_framework::cryptography::session_key_generation::generate_key;

fn generate_session_key_bench(c: &mut Criterion) {
    c.bench_function("session_key_generation::generate_key",
                     |b| b.iter(|| generate_key(black_box(&[32; 32]), black_box(32),
                                                black_box(&mut [32; 40]), black_box(40))));
}

criterion_group!(benches, generate_session_key_bench);
criterion_main!(benches);