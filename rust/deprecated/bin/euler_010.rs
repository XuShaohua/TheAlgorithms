// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate euler;
extern crate test;

/// Problem:
///
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
/// Find the sum of all the primes below two million.

fn method1() -> usize {
    let primes = euler::primes::get_prime_list(2_000_000);
    primes.iter().sum()
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 142913828922));
}
