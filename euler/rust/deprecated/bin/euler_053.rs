// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use num_bigint::BigUint;
use std::ops::Div;

/// Problem:
///
/// There are exactly ten ways of selecting three from five, 12345:
/// 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
///
/// In combinatorics, we use the notation, (5/3)=10.
///
/// In general, (n/r)=n!/ (r!(n−r)!)  , where r≤n, n!=n×(n−1)×...×3×2×1, and 0!=1 .
///
/// It is not until n=23, that a value exceeds one-million: (23/10)=1144066.
///
/// How many, not necessarily distinct, values of (n/r) for 1≤n≤100,
/// are greater than one-million?

fn combination(n: u32, r: u32) -> BigUint {
    let p1 = ((r + 1)..=n).fold(BigUint::from(1_u32), |prod, i| prod * i);
    let p2 = (1..=(n - r)).fold(BigUint::from(1_u32), |prod, i| prod * i);
    p1.div(p2)
}

fn method1() -> u32 {
    let mut count = 0;
    let threshold = BigUint::from(1_000_000_u32);
    for i in 1..=100 {
        for j in 1..=i {
            if combination(i, j) > threshold {
                count += 1;
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
    b.iter(|| assert_eq!(method1(), 4075));
}
