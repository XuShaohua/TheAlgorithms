// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{Criterion, criterion_group, criterion_main};

use sort::selection_sort;
use sort::util::random_ints;

fn criterion_benchmark(c: &mut Criterion) {
    for exp in 1..5 {
        let len: usize = 2 * 10_usize.pow(exp);
        let arr = random_ints(len).expect("Failed to generate random integers");
        let title1 = format!("std_sort {len}");
        let title2 = format!("selection_sort {len}");

        c.bench_function(&title1, |b| b.iter(|| {
            let mut arr1 = arr.clone();
            arr1.sort();
        }));
        c.bench_function(&title2, |b| b.iter(|| {
            let mut arr2 = arr.clone();
            selection_sort(&mut arr2);
        }));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
