// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use num_bigint::BigUint;

/// Problem:
///
/// It is possible to show that the square root of two can be expressed as
/// an infinite continued fraction.
///
///  √2 = 1 + 1/(2+1/(2+1/2+…))
///
/// By expanding this for the first four iterations, we get:
///
///  1+1/2 = 3/2 = 1.5
///  1+1/(2+1/2) = 7/5 = 1.4
///  1+1/(2+1/(2+1/2)) = 17/12 = 1.41666…
///  1+1/(2+1/(2+1(2+1/2))) = 41/29 = 1.41379…
///
///
/// The next three expansions are 99/70, 239/169, and 577/408, but the eighth
/// expansion, 1393/985, is the first example where the number of digits
/// in the numerator exceeds the number of digits in the denominator.
///
/// In the first one-thousand expansions, how many fractions contain
/// a numerator with more digits than the denominator?

fn method1() -> u32 {
    let mut count = 0;
    let mut numerator = BigUint::from(3_u32);
    let mut denominator = BigUint::from(2_u32);
    let mut tmp;
    for _i in 1..1000 {
        if numerator.to_string().len() > denominator.to_string().len() {
            count += 1;
        }
        tmp = denominator.clone();
        denominator += &numerator;
        numerator += tmp * 2_u32;
    }
    count
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 153));
}
