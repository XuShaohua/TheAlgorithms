// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::permutation::Permutation;
use euler::primes::IsPrime;

/// Problem:
///
/// We shall say that an n-digit number is pandigital if it makes use of
/// all the digits 1 to n exactly once. For example, 2143 is a 4-digit
/// pandigital and is also prime.
///
/// What is the largest n-digit pandigital prime that exists?

fn method1() -> usize {
    let mut largest_pandigital_prime = 0;

    for i in 2..=7 {
        let mut digits: Vec<u8> = vec![];
        for j in 1..=i {
            digits.push(j);
        }
        let p = Permutation::new(digits);
        for d in p.into_iter() {
            let mut num: usize = 0;
            for digit in d {
                num = num * 10 + digit as usize;
            }
            if num > largest_pandigital_prime && num.is_prime() {
                println!("> {}, {}", num, largest_pandigital_prime);
                largest_pandigital_prime = num;
            }
        }
    }

    largest_pandigital_prime
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 7652413));
}
