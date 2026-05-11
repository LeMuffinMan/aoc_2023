use aoc_2023::days::{Solution, day01::Day01, day02::Day02};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_day01(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
    c.bench_function("day01 part1", |b| b.iter(|| Day01::part1(&input)));
    c.bench_function("day01 part2", |b| b.iter(|| Day01::part2(&input)));
}

fn bench_day02(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day02.txt").unwrap();
    c.bench_function("day02 part1", |b| b.iter(|| Day02::part1(&input)));
    c.bench_function("day02 part2", |b| b.iter(|| Day02::part2(&input)));
}

criterion_group!(benches, bench_day01, bench_day02);
criterion_main!(benches);
