use aoclib::utils::{md5, DaySolver, PrimeIterator};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn specific_benches(c: &mut Criterion) {
    let input = aoclib::twothousandfifteen::DayEighteen {}.read_input();

    c.bench_function("conway-world-update", |b| {
        let mut w = aoclib::utils::conway::World::from_str(&input, '#', '.').unwrap();
        b.iter(|| w.update())
    });

    c.bench_function("md5", |b| {
        b.iter(|| {
            black_box(md5::calculate_hash("yzbqklnj"));
        });
    });

    c.bench_function("md5_bytes", |b| {
        b.iter(|| {
            black_box(md5::calculate_hash_bytes("yzbqklnj".bytes().collect()));
        });
    });

    c.bench_function("prime_iterator", |b| {
        b.iter(|| {
            let mut it = PrimeIterator::new();
            for _ in 0..100000 {
                black_box(it.next());
            }
        });
    });
}

fn bench_all_challenges(c: &mut Criterion) {
    for year in 2015..=2016 {
        for day in 1..=25 {
            for part in 1..=2 {
                c.bench_function(&format!("{}-{}-{}", year, day, part), |b| {
                    b.iter(|| black_box(aoclib::run_challenge(year, day, part)));
                });
            }
        }
    }
}

criterion_group!(specific, specific_benches);
criterion_group!(challenges, bench_all_challenges);
criterion_main!(specific, challenges);
