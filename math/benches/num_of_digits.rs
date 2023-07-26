// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math::num_of_digits::{num_digits, num_digits_fast, num_digits_faster};

const LARGE_NUMS: &[i64] = &[262_144, 1_125_899_906, 1_267_650_600_228_229_404];

pub fn num_digits_bench(c: &mut Criterion) {
    c.bench_function("math::num_digits-0", |b| {
        b.iter(|| num_digits(black_box(LARGE_NUMS[0])))
    });
    c.bench_function("math::num_digits-1", |b| {
        b.iter(|| num_digits(black_box(LARGE_NUMS[1])))
    });
    c.bench_function("math::num_digits-2", |b| {
        b.iter(|| num_digits(black_box(LARGE_NUMS[2])))
    });
}

pub fn num_digits_fast_bench(c: &mut Criterion) {
    c.bench_function("math::num_digits_fast-0", |b| {
        b.iter(|| num_digits_fast(black_box(LARGE_NUMS[0])))
    });
    c.bench_function("math::num_digits_fast-1", |b| {
        b.iter(|| num_digits_fast(black_box(LARGE_NUMS[1])))
    });
    c.bench_function("math::num_digits_fast-2", |b| {
        b.iter(|| num_digits_fast(black_box(LARGE_NUMS[2])))
    });
}

pub fn num_digits_faster_bench(c: &mut Criterion) {
    c.bench_function("math::num_digits_faster-0", |b| {
        b.iter(|| num_digits_faster(black_box(LARGE_NUMS[0])))
    });
    c.bench_function("math::num_digits_faster-1", |b| {
        b.iter(|| num_digits_faster(black_box(LARGE_NUMS[1])))
    });
    c.bench_function("math::num_digits_faster-2", |b| {
        b.iter(|| num_digits_faster(black_box(LARGE_NUMS[2])))
    });
}

criterion_group!(
    benches,
    num_digits_bench,
    num_digits_fast_bench,
    num_digits_faster_bench
);
criterion_main!(benches);
