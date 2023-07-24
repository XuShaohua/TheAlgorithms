// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math::fibonacci::fib;

pub fn fib_bench(c: &mut Criterion) {
    c.bench_function("math::fib-80", |b| b.iter(|| fib(black_box(80))));
}

criterion_group!(benches, fib_bench);
criterion_main!(benches);
