// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::primes::GetFactors;

/// Problem:
///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than
/// n which divide evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair
/// and each of a and b are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22,
/// 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284
/// are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.

fn method1() -> usize {
    const MAX: usize = 10_000;

    let mut divisors: [usize; MAX + 1] = [0; MAX + 1];

    for i in 2..=MAX {
        let divisor = i.get_factors().iter().sum();
        divisors[i] = divisor;
    }

    let mut sum = 0;
    for (index, divisor) in divisors.iter().enumerate() {
        if index != *divisor && divisor <= &MAX && divisors[*divisor] == index {
            sum += index;
        }
    }

    sum
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 31626));
}
