// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use sort::merge_sort::{bottom_up_merge_sort, insertion_merge_sort, shell_merge_sort, three_way_merge_sort, topdown_merge_sort};
use sort::util::random_ints;

fn criterion_benchmark(c: &mut Criterion) {
    for exp in 1..7 {
        let len: usize = 2 * 10_usize.pow(exp);
        let arr = random_ints(len).expect("Failed to generate random integers");
        let title1 = format!("std_sort_for_merge_sort {len}");
        let title2 = format!("topdown_merge_sort {len}");
        let title3 = format!("insertion_merge_sort {len}");
        let title4 = format!("shell_merge_sort {len}");
        let title5 = format!("bottom_up_merge_sort {len}");
        let title6 = format!("three_way_merge_sort {len}");
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
                topdown_merge_sort(&mut arr2);
                assert_eq!(arr2, arr_sorted);
            })
        });
        c.bench_function(&title3, |b| {
            b.iter(|| {
                let mut arr3 = arr.clone();
                insertion_merge_sort(&mut arr3);
                assert_eq!(arr3, arr_sorted);
            })
        });
        c.bench_function(&title4, |b| {
            b.iter(|| {
                let mut arr4 = arr.clone();
                shell_merge_sort(&mut arr4);
                assert_eq!(arr4, arr_sorted);
            })
        });
        c.bench_function(&title5, |b| {
            b.iter(|| {
                let mut arr5 = arr.clone();
                bottom_up_merge_sort(&mut arr5);
                assert_eq!(arr5, arr_sorted);
            })
        });
        c.bench_function(&title6, |b| {
            b.iter(|| {
                let mut arr6 = arr.clone();
                three_way_merge_sort(&mut arr6);
                assert_eq!(arr6, arr_sorted);
            })
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = criterion_benchmark
);
criterion_main!(benches);
