// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rand::Rng;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortVariant {
    Insertion,
    Selection,
    Shell,
}

fn time_sort(array: &mut [f64], variant: SortVariant) -> u128 {
    let instance = Instant::now();
    match variant {
        SortVariant::Insertion => sorts::insertion_sort(array),
        SortVariant::Selection => sorts::selection_sort(array),
        SortVariant::Shell => sorts::shell_sort(array),
    }
    instance.elapsed().as_nanos()
}

fn time_random_input(variant: SortVariant, len: usize, times: i32) -> u128 {
    let mut total = 0;
    let mut rng = rand::thread_rng();
    let mut array = vec![0.0; len];
    for _t in 0..times {
        for item in array.iter_mut() {
            *item = rng.gen();
        }
        total += time_sort(&mut array, variant);
    }

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Usage: number times");
    }
    let num: usize = args[1].parse().expect("Invalid total length");
    let times: i32 = args[2].parse().expect("Invalid times");
    let t1 = time_random_input(SortVariant::Insertion, num, times);
    let t2 = time_random_input(SortVariant::Selection, num, times);
    let t3 = time_random_input(SortVariant::Shell, num, times);

    println!("Insertion: {t1}, Selection: {t2}, ShellSort: {t3}");
    let ratio1: f64 = (t1 as f64) / (t3 as f64);
    let ratio2: f64 = t2 as f64 / t3 as f64;
    println!("Insertion ratio: {ratio1:.2}, Selection ratio: {ratio2:.2}, Shell ratio: 1.0");
}
