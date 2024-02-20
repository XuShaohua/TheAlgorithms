// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate euler;
extern crate test;

use std::collections::HashSet;
use std::iter::FromIterator;

/// Problem:
///
/// The number, 197, is called a circular prime because all rotations of
/// the digits: 197, 971, and 719, are themselves prime.
/// There are thirteen such primes below 100:
///     2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
/// How many circular primes are there below one million?

fn method1() -> u32 {
    const MAX: usize = 1_000_000;
    let mut circular_count = 0;
    let prime_list = euler::primes::get_prime_list(MAX);
    let primes: HashSet<usize> = HashSet::from_iter(prime_list.into_iter());
    let is_circurlar_prime = |prime: usize| -> bool {
        let rotate_digits = if prime > 100_000 {
            5
        } else if prime > 10_000 {
            4
        } else if prime > 1_000 {
            3
        } else if prime > 100 {
            2
        } else if prime > 10 {
            1
        } else {
            0
        };

        let mut result = true;
        let mut p = prime;
        let rotate = 10_usize.pow(rotate_digits);
        for _i in 0..rotate_digits {
            let quotient = p / rotate;
            // Skip prime numbers which contain even digits
            if quotient == 0
                || quotient == 2
                || quotient == 4
                || quotient == 5
                || quotient == 6
                || quotient == 8
            {
                result = false;
                break;
            }
            // rotate left
            p = p % rotate * 10 + quotient;
            if !primes.contains(&p) {
                result = false;
                break;
            }
        }
        result
    };

    for prime in &primes {
        if is_circurlar_prime(*prime) {
            //println!("> prime: {}", prime);
            circular_count += 1;
        }
    }

    circular_count
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 55));
}
