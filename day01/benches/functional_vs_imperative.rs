use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("extract_number", |b| {
        b.iter(|| day01::extract_number(black_box("abc1def2ghi3")));
    });

    c.bench_function("extract_number2", |b| {
        b.iter(|| day01::extract_number2(black_box("abc1def2ghi3")));
    });

    c.bench_function("extract_number3", |b| {
        b.iter(|| day01::extract_number3(black_box("abc1def2ghi3")));
    });

    c.bench_function("extract_number4", |b| {
        b.iter(|| day01::extract_number4(black_box("abc1def2ghi3")));
    });

    c.bench_function("extract_number5", |b| {
        b.iter(|| day01::extract_number5(black_box("abc1def2ghi3")));
    });

    c.bench_function("extract_number6", |b| {
        b.iter(|| day01::extract_number6(black_box("abc1def2ghi3")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
