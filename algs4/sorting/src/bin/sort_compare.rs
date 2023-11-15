// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rand::Rng;
use std::time::Instant;

fn time_algo(algorithm: &str, array: &mut Vec<f64>) -> u128 {
    let instance = Instant::now();
    match algorithm {
        "Insertion" => sort::insertion_sort(array),
        "Selection" => sort::selection_sort(array),
        "Shell" => sort::shell_sort(array),
        _ => panic!("Invalid algorithm: {}", algorithm),
    }
    return instance.elapsed().as_nanos();
}

fn time_random_input(algorithm: &str, len: usize, times: i32) -> u128 {
    let mut total = 0;
    let mut rng = rand::thread_rng();
    let mut array = vec![0.0; len];
    for _t in 0..times {
        for i in 0..len {
            array[i] = rng.gen();
        }
        total += time_algo(algorithm, &mut array);
    }

    return total;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        panic!("Usage: algo1 algo2 number times");
    }
    let algo1 = &args[1];
    let algo2 = &args[2];
    let num: usize = args[3].parse().expect("Invalid total length");
    let times: i32 = args[4].parse().expect("Invalid times");
    let t1 = time_random_input(algo1, num, times);
    let t2 = time_random_input(algo2, num, times);

    println!("t1: {}, t2: {}", t1, t2);
    let times: f64 = t2 as f64 / t1 as f64;
    println!("{} is {:.2} times faster than {}", algo1, times, algo2);
}
