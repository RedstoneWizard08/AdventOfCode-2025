use criterion::{Criterion, criterion_group, criterion_main};

fn bench_part_1(c: &mut Criterion) {
    c.bench_function("problem 5 - part 1", |b| b.iter(|| problem5::part1::main()));
}

fn bench_part_2(c: &mut Criterion) {
    c.bench_function("problem 5 - part 2", |b| b.iter(|| problem5::part2::main()));
}

criterion_group!(benches, bench_part_1, bench_part_2);
criterion_main!(benches);
