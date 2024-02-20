// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::primes::GetFactors;

/// Problem:
///
/// A perfect number is a number for which the sum of its proper divisors
/// is exactly equal to the number. For example, the sum of the proper divisors
/// of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect
/// number.
///
/// A number n is called deficient if the sum of its proper divisors is
/// less than n and it is called abundant if this sum exceeds n.
///
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest
/// number that can be written as the sum of two abundant numbers is 24.
/// By mathematical analysis, it can be shown that all integers greater than
/// 28123 can be written as the sum of two abundant numbers. However,
/// this upper limit cannot be reduced any further by analysis even though
/// it is known that the greatest number that cannot be expressed as the sum
/// of two abundant numbers is less than this limit.
///
/// Find the sum of all the positive integers which cannot be written as
/// the sum of two abundant numbers.

fn method1() -> u32 {
    // TODO(Shaohua): Tuning method1
    let max = 28123;
    let mut buf = Vec::new();
    let abundant_nums: Vec<u32> = (2_u32..max)
        .filter(|i| {
            i.get_factors_cache(&mut buf);
            let sum: u32 = buf.iter().sum();
            &sum > i
        })
        .collect();

    let mut sum = 0;
    for i in 1..=max {
        let half = i / 2 + 1;
        let mut exists = false;
        for j in &abundant_nums {
            if j > &half {
                break;
            }
            let d = i - j;
            if abundant_nums.binary_search(&d).is_ok() {
                exists = true;
                break;
            }
        }
        if !exists {
            sum += i;
        }
    }

    sum
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 4179871));
}
