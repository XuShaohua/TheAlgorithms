// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::digits::GetDigits;

/// Problem:
///
/// The Fibonacci sequence is defined by the recurrence relation:
///     Fn = F_(n−1) + F_(n−2), where F1 = 1 and F2 = 1.
/// Hence the first 12 terms will be:
///     F1 = 1
///     F2 = 1
///     F3 = 2
///     F4 = 3
///     F5 = 5
///     F6 = 8
///     F7 = 13
///     F8 = 21
///     F9 = 34
///     F10 = 55
///     F11 = 89
///     F12 = 144
/// The 12th term, F12, is the first term to contain three digits.
/// What is the index of the first term in the Fibonacci sequence to
/// contain 1000 digits?

/// f1(n) = 9! * n;
/// f2(n) = 10^n - 1;
/// 9_999_999 is upper bound.
const UPPER_BOND: usize = 7;

fn method1() -> u64 {
    let mut product = 1;
    let mut factorials: [u64; 10] = [1; 10];
    for i in 1..=9 {
        product *= i;
        factorials[i] = product as u64;
    }

    let mut curious_nums = vec![];
    let mut digits = Vec::with_capacity(UPPER_BOND);

    for i in 1_u64..(UPPER_BOND as u64 * factorials[9]) {
        digits.clear();
        let _num_digits = i.get_digits(&mut digits);
        let sum: u64 = digits.iter().map(|&digit| factorials[digit as usize]).sum();
        if i == sum {
            println!(">> {}", i);
            curious_nums.push(i);
        }
    }

    let s: u64 = curious_nums.into_iter().sum();
    s - 1 - 2
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 40730));
}
