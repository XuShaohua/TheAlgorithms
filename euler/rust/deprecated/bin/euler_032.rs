// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use std::collections::HashSet;

/// Problem:
///
/// We shall say that an n-digit number is pandigital if it makes use of
/// all the digits 1 to n exactly once; for example, the 5-digit number, 15234,
/// is 1 through 5 pandigital.
///
/// The product 7254 is unusual, as the identity, 39 × 186 = 7254,
/// containing multiplicand, multiplier, and product is 1 through 9 pandigital.
///
/// Find the sum of all products whose multiplicand/multiplier/product identity
/// can be written as a 1 through 9 pandigital.
/// HINT: Some products can be obtained in more than one way so be sure
/// to only include it once in your sum.

fn method1() -> u32 {
    let mut products: HashSet<u32> = HashSet::new();
    let mut digits: [usize; 10] = [0; 10];
    let mut is_pandigitals = |mut i: u32, mut j: u32, mut p: u32| -> bool {
        if p > 9999 || p < 1000 {
            return false;
        }

        for i in 0..digits.len() {
            digits[i] = 0;
        }

        let mut r: usize;
        let mut count = 0;
        while i > 0 {
            r = (i % 10) as usize;
            if r == 0 || digits[r] != 0 {
                return false;
            }
            digits[r] = r;
            count += 1;
            i /= 10;
        }
        while j > 0 {
            r = (j % 10) as usize;
            if r == 0 || digits[r] != 0 {
                return false;
            }
            digits[r] = r;
            count += 1;
            j /= 10;
        }
        while p > 0 {
            r = (p % 10) as usize;
            if r == 0 || digits[r] != 0 {
                return false;
            }
            digits[r] = r;
            count += 1;
            p /= 10;
        }

        count == 9
    };

    for i in 1..999 {
        for j in i..9999 {
            let p = i * j;
            if is_pandigitals(i, j, p) {
                products.insert(p);
            }
        }
    }

    products.into_iter().sum()
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 45228));
}
