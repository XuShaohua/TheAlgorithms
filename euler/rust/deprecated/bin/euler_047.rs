// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate euler;
extern crate test;

/// Problem:
///
/// The first two consecutive numbers to have two distinct prime factors are:
///
///     14 = 2 × 7
///     15 = 3 × 5
///
/// The first three consecutive numbers to have three distinct prime factors are:
///
///     644 = 2² × 7 × 23
///     645 = 3 × 5 × 43
///     646 = 2 × 17 × 19.
///
/// Find the first four consecutive integers to have four distinct
/// prime factors each. What is the first of these numbers?

fn method1() -> usize {
    let primes = euler::primes::get_prime_list(100_000);
    let mut count = 0;
    for i in 2.. {
        let factors = euler::primes::get_prime_factors(i, &primes);
        if factors.len() != 4 {
            count = 0;
        } else {
            count += 1;
            if count == 4 {
                return i - 3;
            }
        }
    }

    0
}

fn method2() -> usize {
    let primes = euler::primes::get_prime_list(100_000);
    let mut count = 0;
    for i in 2.. {
        let factors = euler::primes::get_prime_factor_num(i, &primes);
        if factors != 4 {
            count = 0;
        } else {
            count += 1;
            if count == 4 {
                return i - 3;
            }
        }
    }

    0
}

fn method3() -> usize {
    let primes = euler::primes::get_prime_list(100_000);
    let mut i = 2;
    loop {
        let factors = euler::primes::get_prime_factor_num(i, &primes);
        if factors != 4 {
            i += 1;
            continue;
        }

        let factors = euler::primes::get_prime_factor_num(i + 3, &primes);
        if factors != 4 {
            i += 4;
            continue;
        }

        let factors = euler::primes::get_prime_factor_num(i + 2, &primes);
        if factors != 4 {
            i += 3;
            continue;
        }

        let factors = euler::primes::get_prime_factor_num(i + 1, &primes);
        if factors != 4 {
            i += 2;
            continue;
        } else {
            return i;
        }
    }
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
    println!("method3: {}", method3());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 134043));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 134043));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(), 134043));
}
