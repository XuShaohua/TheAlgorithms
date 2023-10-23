// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate euler;
extern crate test;

/// Problem:
///
/// The number 3797 has an interesting property. Being prime itself,
/// it is possible to continuously remove digits from left to right,
/// and remain prime at each stage: 3797, 797, 97, and 7.
/// Similarly we can work from right to left: 3797, 379, 37, and 3.
///
/// Find the sum of the only eleven primes that are both truncatable from
/// left to right and right to left.
///
/// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

fn method1() -> usize {
    let primes = euler::primes::get_prime_list(1_000_000);
    let mut truncatable_primes: Vec<usize> = Vec::with_capacity(11);

    let is_truncatable_prime = |prime: usize| -> bool {
        let mut p = prime;
        while p >= 10 {
            p /= 10;
            if primes.binary_search(&p).is_err() {
                return false;
            }
        }
        if primes.binary_search(&p).is_err() {
            return false;
        }

        let mut base = 10;
        let mut p;
        while prime >= base {
            p = prime % base;
            base *= 10;
            if primes.binary_search(&p).is_err() {
                return false;
            }
        }

        true
    };

    for prime in &primes {
        if prime > &10 && is_truncatable_prime(*prime) {
            println!("> #{}, {}", truncatable_primes.len() + 1, prime);
            truncatable_primes.push(*prime);
        }
    }
    truncatable_primes.iter().sum()
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 748317));
}
