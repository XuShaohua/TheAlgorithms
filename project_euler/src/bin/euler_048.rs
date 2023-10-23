// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate num_bigint;
extern crate test;

use num_bigint::BigUint;
use num_traits::pow::Pow;

/// Problem:
///
/// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
///
/// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.

fn method1() -> String {
    const MODULE: u64 = 10_000_000_000;
    let mut sum: u64 = 0;
    for i in 1..=1000 {
        let mut product = 1;
        for _j in 0..i {
            product *= i;
            product %= MODULE;
            if product == 0 {
                break;
            }
        }
        sum += product;
    }

    let mut s = sum.to_string();
    s.split_off(s.len() - 10)
}

fn method2() -> String {
    let mut sum = BigUint::default();
    for i in 1_u32..=1000 {
        let num = BigUint::from(i).pow(i);
        sum += num;
    }
    let mut s = sum.to_string();
    s.split_off(s.len() - 10)
}

fn main() {
    println!("last 10 digits: {}", method1());
    println!("last 10 digits: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), "9110846700"));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), "9110846700"));
}
