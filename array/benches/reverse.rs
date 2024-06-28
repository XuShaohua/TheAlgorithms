// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use array::reverse::reverse_array;

fn criterion_benchmark(c: &mut Criterion) {
    let mut arr_20: Vec<i32> = (0..black_box(20)).collect();
    c.bench_function("reverse_std 20", |b| b.iter(|| arr_20.reverse()));
    c.bench_function("reverse_custom 20", |b| {
        b.iter(|| reverse_array(&mut arr_20))
    });
    drop(arr_20);

    let mut arr_200: Vec<i32> = (0..black_box(200)).collect();
    c.bench_function("reverse_std 200", |b| b.iter(|| arr_200.reverse()));
    c.bench_function("reverse_custom 200", |b| {
        b.iter(|| reverse_array(&mut arr_200))
    });
    drop(arr_200);

    let mut arr_2k: Vec<i32> = (0..black_box(2000)).collect();
    c.bench_function("reverse_std 2k", |b| b.iter(|| arr_2k.reverse()));
    c.bench_function("reverse_custom 2k", |b| {
        b.iter(|| reverse_array(&mut arr_2k))
    });
    drop(arr_2k);

    let mut arr_20k: Vec<i32> = (0..black_box(20_000)).collect();
    c.bench_function("reverse_std 20k", |b| b.iter(|| arr_20k.reverse()));
    c.bench_function("reverse_custom 20k", |b| {
        b.iter(|| reverse_array(&mut arr_20k))
    });
    drop(arr_20k);

    let mut arr_200k: Vec<i32> = (0..black_box(200_000)).collect();
    c.bench_function("reverse_std 200k", |b| b.iter(|| arr_200k.reverse()));
    c.bench_function("reverse_custom 200k", |b| {
        b.iter(|| reverse_array(&mut arr_200k))
    });
    drop(arr_200k);

    let mut arr_2m: Vec<i32> = (0..black_box(2_000_000)).collect();
    c.bench_function("reverse_std 2m", |b| b.iter(|| arr_2m.reverse()));
    c.bench_function("reverse_custom 2m", |b| {
        b.iter(|| reverse_array(&mut arr_2m))
    });
    drop(arr_2m);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
