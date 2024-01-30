// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// The sum of the squares of the first ten natural numbers is,
///
///     1^2 + 2^2 + ... + 10^2 = 385
///
/// The square of the sum of the first ten natural numbers is,
///     (1 + 2 + ... + 10)^2 = 55^2 = 3025
///
/// Hence the difference between the sum of the squares of the first
/// ten natural numbers and the square of the sum is 3025−385=2640 .
///
/// Find the difference between the sum of the squares of the first
/// one hundred natural numbers and the square of the sum.

fn method1(max_num: i64) -> i64 {
    let mut square_sum = 0;
    for i in 1..=max_num {
        square_sum += i * i;
    }

    let mut sum = 0;
    for i in 1..=max_num {
        sum += i;
    }
    sum * sum - square_sum
}

fn method2(max_num: i64) -> i64 {
    let square_sum: i64 = (1..=max_num).map(|i| i * i).sum();
    let sum: i64 = (1..=max_num).sum();
    sum * sum - square_sum
}

fn main() {
    let max_num = 100;
    println!("result: {}", method1(max_num));
    println!("result: {}", method2(max_num));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(100), 25164150));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(100), 25164150));
}
