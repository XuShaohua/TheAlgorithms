// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{Criterion, criterion_group, criterion_main};

use sort::selection_sort::{recursive_selection_sort, selection_sort, stable_selection_sort, two_way_selection_sort};
use sort::util::random_ints;

fn criterion_benchmark(c: &mut Criterion) {
    for exp in 1..5 {
        let len: usize = 2 * 10_usize.pow(exp);
        let arr = random_ints(len).expect("Failed to generate random integers");
        let title1 = format!("std_sort_for_selection_sort {len}");
        let title2 = format!("selection_sort {len}");
        let title3 = format!("recursive_selection_sort {len}");
        let title4 = format!("two_way_selection_sort {len}");
        let title5 = format!("stable_selection_sort {len}");
        let mut arr_sorted = arr.clone();
        arr_sorted.sort();

        c.bench_function(&title1, |b| b.iter(|| {
            let mut arr1 = arr.clone();
            arr1.sort();
            assert_eq!(arr1, arr_sorted);
        }));
        c.bench_function(&title2, |b| b.iter(|| {
            let mut arr2 = arr.clone();
            selection_sort(&mut arr2);
            assert_eq!(arr2, arr_sorted);
        }));
        c.bench_function(&title3, |b| b.iter(|| {
            let mut arr3 = arr.clone();
            recursive_selection_sort(&mut arr3);
            assert_eq!(arr3, arr_sorted);
        }));
        c.bench_function(&title4, |b| b.iter(|| {
            let mut arr4 = arr.clone();
            two_way_selection_sort(&mut arr4);
            assert_eq!(arr4, arr_sorted);
        }));
        c.bench_function(&title5, |b| b.iter(|| {
            let mut arr5 = arr.clone();
            stable_selection_sort(&mut arr5);
            assert_eq!(arr5, arr_sorted);
        }));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
