// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{Criterion, criterion_group, criterion_main};

use sort::bubble_sort;
use sort::util::random_ints;

fn criterion_benchmark(c: &mut Criterion) {
    for exp in 1..6 {
        let len: usize = 2 * 10_usize.pow(exp);
        let mut arr1 = random_ints(len).expect("Failed to generate random integers");
        let mut arr2 = arr1.clone();
        let mut arr3 = arr1.clone();
        let title1 = format!("std_sort {len}");
        let title2 = format!("std_sort_unstable {len}");
        let title3 = format!("bubble_sort {len}");

        c.bench_function(&title1, |b| b.iter(|| arr1.sort()));
        c.bench_function(&title2, |b| b.iter(|| arr2.sort_unstable()));
        c.bench_function(&title3, |b| b.iter(|| bubble_sort(&mut arr3)));
        assert_eq!(arr1, arr2);
        assert_eq!(arr1, arr3);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
