// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use bitwise::count_number_of_one_bits::{brian_kernighans_algorithm, builtin, modulo_operator};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn brian_bench(c: &mut Criterion) {
    c.bench_function("bit::count_ones-brian-25", |b| {
        b.iter(|| brian_kernighans_algorithm(black_box(25)))
    });
    c.bench_function("bit::count_ones-brian-37", |b| {
        b.iter(|| brian_kernighans_algorithm(black_box(37)))
    });
    c.bench_function("bit::count_ones-brian-58", |b| {
        b.iter(|| brian_kernighans_algorithm(black_box(58)))
    });
}

pub fn modulo_bench(c: &mut Criterion) {
    c.bench_function("bit::count_ones-modulo-25", |b| {
        b.iter(|| modulo_operator(black_box(25)))
    });
    c.bench_function("bit::count_ones-modulo-37", |b| {
        b.iter(|| modulo_operator(black_box(37)))
    });
    c.bench_function("bit::count_ones-modulo-58", |b| {
        b.iter(|| modulo_operator(black_box(58)))
    });
}

pub fn builtin_bench(c: &mut Criterion) {
    c.bench_function("bit::count_ones-builtin-25", |b| {
        b.iter(|| builtin(black_box(25)))
    });
    c.bench_function("bit::count_ones-builtin-37", |b| {
        b.iter(|| builtin(black_box(37)))
    });
    c.bench_function("bit::count_ones-builtin-58", |b| {
        b.iter(|| builtin(black_box(58)))
    });
}

criterion_group!(benches, brian_bench, modulo_bench, builtin_bench,);
criterion_main!(benches);
