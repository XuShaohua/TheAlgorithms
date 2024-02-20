// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::primes::get_prime_list;

/// Problem:
/// Euler discovered the remarkable quadratic formula:
///     n^2 + n + 41
///
/// It turns out that the formula will produce 40 primes for the consecutive
/// integer values 0 ≤ n ≤ 39. However, when n = 40,
/// 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly
/// when n = 41,41^2 + 41 + 41 is clearly divisible by 41.
///
/// The incredible formula n^2 − 79^n + 1601 was discovered, which produces
/// 80 primes for the consecutive values 0 ≤ n ≤ 79. The product of
/// the coefficients, −79 and 1601, is −126479.
///
/// Considering quadratics of the form:
///
///     n^2 + an + b, where |a| < 1000 and |b| ≤ 1000
///     where |n| is the modulus/absolute value of n
///     e.g. |11| = 11 and |−4| = 4
///
/// Find the product of the coefficients, a and b, for the quadratic expression
/// that produces the maximum number of primes for consecutive values of n,
/// starting with n = 0.

fn method1() -> i32 {
    let prime_list = get_prime_list(2_300_000);
    let mut coefficients = 0;
    let mut max_num_primes = 0;

    let is_product = |n: i32| prime_list.binary_search(&(n as usize)).is_ok();
    let b_primes = get_prime_list(1000);

    for a in -999..1000_i32 {
        for b in &b_primes {
            let mut num_primes = 0;
            let b = *b as i32;
            for i in 0.. {
                let product = i * i + a * i + b;
                if is_product(product) {
                    num_primes += 1;
                } else {
                    break;
                }
            }

            if num_primes > max_num_primes {
                max_num_primes = num_primes;
                coefficients = a * b;
            }
        }
    }
    coefficients
}

fn method2() -> i32 {
    let prime_list = get_prime_list(2_300_000);
    let mut coefficients = 0;
    let mut max_num_primes = 0;

    let is_product = |n: i32| prime_list.binary_search(&(n as usize)).is_ok();

    for a in -999..1000_i32 {
        for b in 2..1000 {
            let mut num_primes = 0;
            for i in 0.. {
                let product = i * i + a * i + b;
                if is_product(product) {
                    num_primes += 1;
                } else {
                    break;
                }
            }

            if num_primes > max_num_primes {
                max_num_primes = num_primes;
                coefficients = a * b;
            }
        }
    }
    coefficients
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), -59231));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), -59231));
}
