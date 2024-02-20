// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate euler;
extern crate test;

use euler::primes::get_prime_list;

/// Problem:
///
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143 ?

const LARGEST_PRIME: u64 = 600851475143;

fn method1(num: u64) -> u64 {
    // Largest possible prime factor of an integer is its square root.
    let sqrt: usize = (num as f64).sqrt().ceil() as usize;

    // Now get prime list smaller than square root.
    let prime_list = get_prime_list(sqrt);
    for prime in prime_list.into_iter().rev() {
        if num % (prime as u64) == 0 {
            return prime as u64;
        }
    }

    0
}

fn method2(num: u64) -> u64 {
    for i in 2..=num {
        if num % i == 0 {
            return if num == i { num } else { method2(num / i) };
        }
    }
    0
}

fn method3(mut num: u64) -> u64 {
    let mut i = 2;
    while i < num {
        if num % i == 0 {
            num /= i;
        }
        i += 1;
    }
    i
}

fn method4(mut num: u64) -> u64 {
    let mut i = 2;
    while i <= num {
        if num % i == 0 {
            num /= i;
        } else {
            if i == 2 {
                i += 1;
            } else {
                i += 2;
            }
        }
    }
    i
}

fn main() {
    println!("method1: {}", method1(LARGEST_PRIME));
    println!("method2: {}", method2(LARGEST_PRIME));
    println!("method3: {}", method3(LARGEST_PRIME));
    println!("method4: {}", method4(LARGEST_PRIME));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(LARGEST_PRIME), 6857));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(LARGEST_PRIME), 6857));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(LARGEST_PRIME), 6857));
}

#[bench]
fn bench_method4(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method4(LARGEST_PRIME), 6857));
}
