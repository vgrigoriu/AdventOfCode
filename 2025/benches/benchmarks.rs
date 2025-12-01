use criterion::{Criterion, criterion_group, criterion_main};
use aoc2025::dial::Dial;

fn benchmark_dial_r(c: &mut Criterion) {
    c.bench_function("apply R", |b| {
        b.iter(|| {
            let mut dial = Dial::new();
            dial.apply("R1000");
        })
    });
}

fn benchmark_dial_l(c: &mut Criterion) {
    c.bench_function("apply L", |b| {
        b.iter(|| {
            let mut dial = Dial::new();
            dial.apply("L1000");
        })
    });
}

criterion_group!(dial_r, benchmark_dial_r);
criterion_group!(dial_l, benchmark_dial_l);
criterion_main!(dial_r, dial_l);