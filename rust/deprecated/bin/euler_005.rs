// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use std::cmp::max;

use euler::primes::{get_prime_factors, get_prime_list, PrimeFactor};

/// Problem:
///
/// 2520 is the smallest number that can be divided by each of the numbers from
/// 1 to 10 without any remainder. What is the smallest positive number
/// that is evenly divisible by all of the numbers from 1 to 20?

fn method1(max_num: usize) -> usize {
    let ls = get_prime_list(max_num);
    let mut minimum_factors = Vec::with_capacity(ls.len());
    for factor in &ls {
        minimum_factors.push(PrimeFactor {
            num: *factor,
            count: 0,
        });
    }

    for i in 2..=max_num {
        let factors = get_prime_factors(i, &ls);
        for factor in &factors {
            for m in &mut minimum_factors {
                if m.num == factor.num {
                    m.count = max(m.count, factor.count);
                }
            }
        }
    }

    minimum_factors.iter().fold(1, |p, f| p * f.num.pow(f.count as u32))
}

fn main() {
    println!("method1 {}", method1(20));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(20), 232792560));
}