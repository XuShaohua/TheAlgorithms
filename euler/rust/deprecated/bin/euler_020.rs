// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// n! means n × (n − 1) × ... × 3 × 2 × 1
///
/// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
/// and the sum of the digits in the number 10! is
/// 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
///
/// Find the sum of the digits in the number 100!

fn method1() -> u16 {
    const MAX: usize = 1024;
    let mut digits: [u16; MAX] = [0; MAX];
    digits[0] = 1;
    for i in 1..=100 {
        let mut quotient = 0;
        for j in 0..MAX {
            quotient += digits[j] * i;
            digits[j] = quotient % 10;
            quotient /= 10;
        }
    }

    digits.iter().sum()
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 648));
}
