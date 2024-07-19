// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use sort::bucket_sort::{bucket_sort, shell_bucket_sort};
use sort::util::random_ints_in_range;

fn criterion_benchmark(c: &mut Criterion) {
    for exp in 1..7 {
        let len: usize = 2 * 10_usize.pow(exp);
        let max = (len as i32) * 2;
        let arr = random_ints_in_range(len, 0, max).expect("Failed to generate random integers");
        let title1 = format!("std_sort_for_bucket_sort {len}");
        let title2 = format!("bucket_sort {len}");
        let title3 = format!("shell_bucket_sort {len}");
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
                bucket_sort(&mut arr2);
                assert_eq!(arr2, arr_sorted);
            })
        });
        c.bench_function(&title3, |b| {
            b.iter(|| {
                let mut arr3 = arr.clone();
                shell_bucket_sort(&mut arr3);
                assert_eq!(arr3, arr_sorted);
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
