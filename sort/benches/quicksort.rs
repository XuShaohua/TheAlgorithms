// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use sort::quicksort::quicksort;
use sort::util::random_ints;

fn criterion_benchmark(c: &mut Criterion) {
    for exp in 1..7 {
        let len: usize = 2 * 10_usize.pow(exp);
        let arr = random_ints(len).expect("Failed to generate random integers");
        let title1 = format!("std_sort_for_quicksort {len}");
        let title2 = format!("quicksort {len}");
        let title3 = format!("head_quicksort {len}");
        let title4 = format!("two_pointer_quicksort {len}");
        let mut arr_sorted = arr.clone();
        arr_sorted.sort();

        c.bench_function(&title1, |b| {
            b.iter(|| {
                let mut arr1 = arr.clone();
                arr1.sort();
                assert_eq!(arr1, arr_sorted);
            })
        });
        c.bench_function(&title2, |b| {
            b.iter(|| {
                let mut arr2 = arr.clone();
                quicksort(&mut arr2);
                assert_eq!(arr2, arr_sorted);
            })
        });
        c.bench_function(&title3, |b| {
            b.iter(|| {
                let mut arr3 = arr.clone();
                quicksort(&mut arr3);
                assert_eq!(arr3, arr_sorted);
            })
        });
        c.bench_function(&title4, |b| {
            b.iter(|| {
                let mut arr4 = arr.clone();
                quicksort(&mut arr4);
                assert_eq!(arr4, arr_sorted);
            })
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
);
criterion_main!(benches);
