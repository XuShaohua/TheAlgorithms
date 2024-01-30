// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// Surprisingly there are only three numbers that can be written as
/// the sum of fourth powers of their digits:
///
///     1634 = 14 + 64 + 34 + 44
///     8208 = 84 + 24 + 04 + 84
///     9474 = 94 + 44 + 74 + 44
///
/// As 1 = 14 is not a sum it is not included.
///
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
///
/// Find the sum of all the numbers that can be written as
/// the sum of fifth powers of their digits.

const FIFTH_POW: [u64; 10] = [
    0,
    1,
    2 * 2 * 2 * 2 * 2,
    3 * 3 * 3 * 3 * 3,
    4 * 4 * 4 * 4 * 4,
    5 * 5 * 5 * 5 * 5,
    6 * 6 * 6 * 6 * 6,
    7 * 7 * 7 * 7 * 7,
    8 * 8 * 8 * 8 * 8,
    9 * 9 * 9 * 9 * 9,
];

/// For n * 9 ^ 5 > 10^n - 1, max value of n is 5.
fn method1() -> u64 {
    const MAX: usize = 10;
    let mut digits: [u8; MAX] = [0; MAX];
    let mut fifth_power_nums = vec![];
    // Skip 1
    digits[0] = 1;
    for i in 2..1_000_000_u64 {
        let mut sum = 1;
        for j in 0..MAX {
            sum += digits[j];
            if sum > 9 {
                digits[j] = 0;
                sum = 1;
            } else {
                digits[j] = sum;
                break;
            }
        }

        let sum: u64 = digits.iter().map(|digit| (*digit as u64).pow(5)).sum();
        if sum == i {
            fifth_power_nums.push(i);
        }
    }
    fifth_power_nums.into_iter().sum()
}

fn method2() -> u64 {
    let mut fifth_power_nums = vec![];
    let mut s;
    for i in 2..1_000_000_u64 {
        s = i.to_string();
        let sum: u64 = s
            .bytes()
            .map(|b| {
                let digit: u64 = (b - b'0') as u64;
                digit.pow(5)
            })
            .sum();
        if sum == i {
            fifth_power_nums.push(i);
        }
    }
    fifth_power_nums.into_iter().sum()
}

fn method3() -> u64 {
    const MAX: usize = 10;
    let mut digits: [u8; MAX] = [0; MAX];
    let mut fifth_power_nums = vec![];
    digits[0] = 1;
    for i in 2..1_000_000_u64 {
        let mut sum = 1;
        for j in 0..MAX {
            sum += digits[j];
            if sum > 9 {
                digits[j] = 0;
                sum = 1;
            } else {
                digits[j] = sum;
                break;
            }
        }

        let sum: u64 = digits.iter().map(|digit| FIFTH_POW[*digit as usize]).sum();
        if sum == i {
            fifth_power_nums.push(i);
        }
    }
    fifth_power_nums.into_iter().sum()
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
    println!("method3: {}", method3());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 443839));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 443839));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(), 443839));
}
