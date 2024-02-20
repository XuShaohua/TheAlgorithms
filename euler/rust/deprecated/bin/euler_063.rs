// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::digits::CountDigits;

/// Problem:
///
/// The 5-digit number, 16807=7^5, is also a fifth power. Similarly,
/// the 9-digit number, 134217728=8^9, is a ninth power.
///
/// How many n-digit positive integers exist which are also an nth power?

fn method1() -> u32 {
    let mut count = 0;
    // Obveriously root number cannot be larger than 10
    for i in 1..10_u128 {
        for j in 1_u32.. {
            let p = i.pow(j);
            let n_digits = p.count_digits() as u32;
            if j == n_digits {
                println!("{} = {}^{}", p, i, j);
                count += 1;
            } else if j > n_digits {
                break;
            }
        }
    }
    count
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 49));
}
