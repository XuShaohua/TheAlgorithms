// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use bitwise::binary_count_trailing_zeros::{builtin, count_trailing_zeros};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn brute_force_bench(c: &mut Criterion) {
    c.bench_function("bit::trailing_zero-brute-25", |b| {
        b.iter(|| count_trailing_zeros(black_box(25)))
    });
    c.bench_function("bit::trailing_zero-brute-37", |b| {
        b.iter(|| count_trailing_zeros(black_box(37)))
    });
    c.bench_function("bit::trailing-zero-brute-58", |b| {
        b.iter(|| count_trailing_zeros(black_box(58)))
    });
}

pub fn builtin_bench(c: &mut Criterion) {
    c.bench_function("bit::trailing_zero-builtin-25", |b| {
        b.iter(|| builtin(black_box(25)))
    });
    c.bench_function("bit::trailing_zero-builtin-37", |b| {
        b.iter(|| builtin(black_box(37)))
    });
    c.bench_function("bit::trailing_zero-builtin-58", |b| {
        b.iter(|| builtin(black_box(58)))
    });
}

criterion_group!(benches, brute_force_bench, builtin_bench,);
criterion_main!(benches);
