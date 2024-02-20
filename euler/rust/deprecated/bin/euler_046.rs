// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::primes::get_prime_list;

/// Problem:
///
/// It was proposed by Christian Goldbach that every odd composite number
/// can be written as the sum of a prime and twice a square.
///
/// 9 = 7 + 2×1^2
/// 15 = 7 + 2×2^2
/// 21 = 3 + 2×3^2
/// 25 = 7 + 2×3^2
/// 27 = 19 + 2×2^2
/// 33 = 31 + 2×1^2
///kip
/// It turns out that the conjecture was false.
///
/// What is the smallest odd composite that cannot be written as
/// the sum of a prime and twice a square?

fn method1() -> usize {
    let primes = get_prime_list(100_000);
    for num in (9..).step_by(2) {
        if primes.binary_search(&num).is_ok() {
            continue;
        }

        let mut ok = true;
        for prime in &primes {
            if prime > &num {
                ok = false;
                break;
            }

            ok = true;
            let remainder = num - prime;
            for i in 1.. {
                let s = 2 * i * i;
                if s > remainder {
                    ok = false;
                    break;
                } else if s == remainder {
                    ok = true;
                    break;
                }
            }

            if ok {
                //println!("num: {}, prime: {}", num, prime);
                break;
            }
        }

        if !ok {
            return num;
        }
    }

    0
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 5777));
}
