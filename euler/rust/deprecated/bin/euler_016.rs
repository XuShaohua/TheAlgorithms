// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
/// What is the sum of the digits of the number 2^1000?

fn method1() -> u32 {
    const MAX: usize = 1000;
    let mut digits: [u8; MAX] = [0; MAX];
    digits[0] = 1;
    for _ in 0..MAX {
        let mut quotient = 0;
        for digit in digits.iter_mut().take(MAX) {
            quotient += *digit * 2;
            if quotient >= 10 {
                *digit = quotient - 10;
                quotient = 1;
            } else {
                *digit = quotient;
                quotient = 0;
            }
        }
    }

    digits.iter().map(|n| *n as u32).sum()
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 1366));
}
