// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use array::rotate::rotate_array_3;

fn criterion_benchmark(c: &mut Criterion) {
    let mut arr_20: Vec<i32> = (0..black_box(20)).collect();
    let k: usize = ((arr_20.len() as f64) * 0.4) as usize;
    let k_isize = k as isize;
    c.bench_function("rotate_std 20", |b| b.iter(|| arr_20.rotate_left(k)));
    c.bench_function("rotate_custom 20", |b| {
        b.iter(|| rotate_array_3(&mut arr_20, k_isize))
    });
    drop(arr_20);

    let mut arr_200: Vec<i32> = (0..black_box(200)).collect();
    let k: usize = ((arr_200.len() as f64) * 0.4) as usize;
    let k_isize = k as isize;
    c.bench_function("rotate_std 200", |b| b.iter(|| arr_200.rotate_left(k)));
    c.bench_function("rotate_custom 200", |b| {
        b.iter(|| rotate_array_3(&mut arr_200, k_isize))
    });
    drop(arr_200);

    let mut arr_2k: Vec<i32> = (0..black_box(2000)).collect();
    let k: usize = ((arr_2k.len() as f64) * 0.4) as usize;
    let k_isize = k as isize;
    c.bench_function("rotate_std 2k", |b| b.iter(|| arr_2k.rotate_left(k)));
    c.bench_function("rotate_custom 2k", |b| {
        b.iter(|| rotate_array_3(&mut arr_2k, k_isize))
    });
    drop(arr_2k);

    let mut arr_20k: Vec<i32> = (0..black_box(20_000)).collect();
    let k: usize = ((arr_20k.len() as f64) * 0.4) as usize;
    let k_isize = k as isize;
    c.bench_function("rotate_std 20k", |b| b.iter(|| arr_20k.rotate_left(k)));
    c.bench_function("rotate_custom 20k", |b| {
        b.iter(|| rotate_array_3(&mut arr_20k, k_isize))
    });
    drop(arr_20k);

    let mut arr_200k: Vec<i32> = (0..black_box(200_000)).collect();
    let k: usize = ((arr_200k.len() as f64) * 0.4) as usize;
    let k_isize = k as isize;
    c.bench_function("rotate_std 200k", |b| b.iter(|| arr_200k.rotate_left(k)));
    c.bench_function("rotate_custom 200k", |b| {
        b.iter(|| rotate_array_3(&mut arr_200k, k_isize))
    });
    drop(arr_200k);

    let mut arr_2m: Vec<i32> = (0..black_box(2_000_000)).collect();
    let k: usize = ((arr_2m.len() as f64) * 0.4) as usize;
    let k_isize = k as isize;
    c.bench_function("rotate_std 2m", |b| b.iter(|| arr_2m.rotate_left(k)));
    c.bench_function("rotate_custom 2m", |b| {
        b.iter(|| rotate_array_3(&mut arr_2m, k_isize))
    });
    drop(arr_2m);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
