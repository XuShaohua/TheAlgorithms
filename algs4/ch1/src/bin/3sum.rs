// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::SystemTime;

use base::ints;

/// O(n) = n^3
fn brute_force(ints: Vec<i32>) -> i32 {
    let mut count = 0;
    let len = ints.len();
    for i in 0..len {
        for j in (i + 1)..len {
            for k in (j + 1)..len {
                if ints[i] + ints[j] + ints[k] == 0 {
                    count += 1;
                }
            }
        }
    }
    count
}

/// O(n) = n^2 * lg(n)
fn sort_and_search(mut ints: Vec<i32>) -> i32 {
    ints.sort();
    let mut count = 0;
    for i in 0..ints.len() {
        for j in (i + 1)..ints.len() {
            let s = 0 - (ints[i] + ints[j]);
            if ints.binary_search(&s).is_ok() {
                count += 1;
            }
        }
    }
    count / 3
}

fn main() {
    let ints = ints::read_ints();
    println!("length of ints: {}", ints.len());
    let ints_cloned = ints.clone();

    let now = SystemTime::now();
    let count = sort_and_search(ints_cloned);
    if let Ok(elapsed) = now.elapsed() {
        println!(
            "[sort_and_search] count: {}, elapsed: {}ms",
            count,
            elapsed.as_millis()
        );
    } else {
        println!("[sort_and_search] count: {}", count);
    }

    let now = SystemTime::now();
    let count = brute_force(ints);
    if let Ok(elapsed) = now.elapsed() {
        println!(
            "[brute_force] count: {}, elapsed: {}ms",
            count,
            elapsed.as_millis()
        );
    } else {
        println!("[brute_force] count: {}", count);
    }
}
