// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::primes::{get_prime_list, IsPrime};

/// Problem:
///
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
/// that the 6th prime is 13.
///
/// What is the 10 001st prime number?

fn method1(nth_prime: u32) -> usize {
    let prime_list = get_prime_list(110_000);
    prime_list[nth_prime as usize - 1]
}

fn method2(nth_prime: u32) -> usize {
    let mut primes = Vec::with_capacity(nth_prime as usize);
    primes.push(2);
    let mut num = 3;
    while primes.len() < nth_prime as usize {
        if num.is_prime() {
            primes.push(num);
        }

        num += 2;
    }

    primes[primes.len() - 1]
}

fn main() {
    let nth_prime = 10001;
    println!("#10001 prime is: {}", method1(nth_prime));
    println!("#10001 prime is: {}", method2(nth_prime));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(10001), 104743));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(10001), 104743));
}
