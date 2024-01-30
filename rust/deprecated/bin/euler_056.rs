// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use num_bigint::BigUint;
use num_traits::pow::Pow;

/// Problem:
///
/// A googol (10100) is a massive number: one followed by one-hundred zeros;
/// 100100 is almost unimaginably large: one followed by two-hundred zeros.
/// Despite their size, the sum of the digits in each number is only 1.
///
/// Considering natural numbers of the form, ab, where a, b < 100,
/// what is the maximum digital sum?

fn method1() -> u32 {
    let mut largest_sum = 0;
    for i in 2..100_u16 {
        for j in 2..100_u16 {
            let p: BigUint = BigUint::from(i).pow(j);
            let len = p
                .to_radix_le(10)
                .iter()
                .map(|num: &u8| -> u32 { *num as u32 })
                .sum();
            if len > largest_sum {
                largest_sum = len;
            }
        }
    }
    largest_sum
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 972));
}
