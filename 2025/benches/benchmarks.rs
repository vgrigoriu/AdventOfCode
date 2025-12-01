use criterion::{Criterion, criterion_group, criterion_main};
use aoc2025::dial::Dial;

fn benchmark_dial(c: &mut Criterion) {
    c.bench_function("Dial::apply", |b| {
        b.iter(|| {
            let mut dial = Dial::new();
            dial.apply("R1000");
        })
    });
}

criterion_group!(dial, benchmark_dial);
criterion_main!(dial);