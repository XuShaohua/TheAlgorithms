// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::gcd::Gcd;

/// Problem:
///
/// The fraction 49/98 is a curious fraction, as an inexperienced mathematician
/// in attempting to simplify it may incorrectly believe that 49/98 = 4/8,
/// which is correct, is obtained by cancelling the 9s.
///
/// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
///
/// There are exactly four non-trivial examples of this type of fraction,
/// less than one in value, and containing two digits in the numerator and
/// denominator.
///
/// If the product of these four fractions is given in its lowest common terms,
/// find the value of the denominator.

fn method1() -> u32 {
    let is_canceling_fraction = |numerator: u32, denominator: u32| -> bool {
        let n1 = numerator / 10;
        let n2 = numerator % 10;
        let d1 = denominator / 10;
        let d2 = denominator % 10;
        (n2 == d1) && (n1 * denominator == d2 * numerator)
    };

    let mut denominators = 1;
    let mut numerators = 1;
    for i in 10..99_u32 {
        for j in (i + 1)..=99 {
            if is_canceling_fraction(i, j) {
                println!("> {} / {}", i, j);
                numerators *= i;
                denominators *= j;
                let common = u32::gcd(numerators, denominators);
                numerators /= common;
                denominators /= common;
            }
        }
    }

    denominators
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 100));
}
