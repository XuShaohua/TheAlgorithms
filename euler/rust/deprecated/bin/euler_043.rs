// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::permutation::Permutation;

/// Problem:
///
/// The number, 1406357289, is a 0 to 9 pandigital number because it is
/// made up of each of the digits 0 to 9 in some order, but it also
/// has a rather interesting sub-string divisibility property.
///
/// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way,
/// we note the following:
///
///     d2d3d4=406 is divisible by 2
///     d3d4d5=063 is divisible by 3
///     d4d5d6=635 is divisible by 5
///     d5d6d7=357 is divisible by 7
///     d6d7d8=572 is divisible by 11
///     d7d8d9=728 is divisible by 13
///     d8d9d10=289 is divisible by 17
///
/// Find the sum of all 0 to 9 pandigital numbers with this property.

fn method1() -> u64 {
    let mut sum = vec![];
    let pairs = [
        (1_usize, 2),
        (2, 3),
        (3, 5),
        (4, 7),
        (5, 11),
        (6, 13),
        (7, 17),
    ];

    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let p = Permutation::new(digits);
    'iterator: for item in p.into_iter() {
        for (i, prime) in &pairs {
            let num = (item[*i] as u64) * 100 + (item[i + 1] as u64) * 10 + item[i + 2] as u64;
            if num % prime != 0 {
                continue 'iterator;
            }
        }

        let mut num: u64 = 0;
        for i in item {
            num = (num * 10) + (i as u64);
        }
        println!("> {}", num);
        sum.push(num);
    }

    sum.into_iter().sum()
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 16695334890));
}
