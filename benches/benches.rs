use aoclib::utils::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn specific_benches(c: &mut Criterion) {
    let input = aoclib::twothousandfifteen::DayEighteen {}.read_input();

    c.bench_function("conway-world-update 2015-18", |b| {
        let mut w = aoclib::utils::conway::World::from_str(&input, '#', '.').unwrap();
        b.iter(|| w.update())
    });

    c.bench_function("md5 2015-4", |b| {
        b.iter(|| {
            black_box(md5::calculate_hash("yzbqklnj"));
        });
    });

    c.bench_function("md5_bytes 2015-4", |b| {
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

    c.bench_function("grid_index_to_flat_index 2015-25 low row and column", |b| {
        b.iter(|| {
            black_box(triangular_matrix_row_column_index_to_flat_index(50, 50));
        });
    });

    c.bench_function(
        "grid_index_to_flat_index 2015-25 high row and column",
        |b| {
            b.iter(|| {
                black_box(triangular_matrix_row_column_index_to_flat_index(5000, 5000));
            });
        },
    );

    c.bench_function(
        "grid_index_to_flat_index 2015-25 iterate up to index 9 million",
        |b| {
            b.iter(|| {
                for row in 1..=3_000 {
                    for column in 1..=3_000 {
                        black_box(triangular_matrix_row_column_index_to_flat_index(
                            row, column,
                        ));
                    }
                }
            });
        },
    );

    c.bench_function(
        "flat_index_to_grid_index 2015-25 iterate up to index 9 million",
        |b| {
            b.iter(|| {
                for index in 1..=9_000_000 {
                    black_box(triangular_matrix_flat_index_to_row_column_index(index));
                }
            });
        },
    );
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
