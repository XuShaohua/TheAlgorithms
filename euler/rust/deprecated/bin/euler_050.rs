// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// The prime 41, can be written as the sum of six consecutive primes:
/// 41 = 2 + 3 + 5 + 7 + 11 + 13
///
/// This is the longest sum of consecutive primes that adds to a prime
/// below one-hundred.
///
/// The longest sum of consecutive primes below one-thousand that adds to
/// a prime, contains 21 terms, and is equal to 953.
///
/// Which prime, below one-million, can be written as the sum
/// of the most consecutive primes?

fn method1() -> usize {
    const MAX: usize = 1_000_000;
    let primes = euler::primes::get_prime_list(MAX);
    let len = primes.len();
    let mut max_consecutive_prime = 0;
    let mut max_consecutive_count = 0;
    for i in 0..len {
        let prime = primes[i];
        for j in 0..i {
            let mut count = 0;
            let mut sum = 0;
            while sum < prime && j + count < i {
                sum += primes[j + count];
                count += 1;
            }
            if count <= max_consecutive_count {
                break;
            }
            if sum == prime {
                if count > max_consecutive_count {
                    max_consecutive_prime = prime;
                    max_consecutive_count = count;
                }
                break;
            }
        }
    }
    max_consecutive_prime
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 997651));
}
