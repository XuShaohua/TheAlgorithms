// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// Starting with the number 1 and moving to the right in a clockwise direction
/// a 5 by 5 spiral is formed as follows:
///
///     21 22 23 24 25
///     20  7  8  9 10
///     19  6  1  2 11
///     18  5  4  3 12
///     17 16 15 14 13
///
/// It can be verified that the sum of the numbers on the diagonals is 101.
///
/// What is the sum of the numbers on the diagonals in a 1001 by 1001
/// spiral formed in the same way?

fn method1() -> u32 {
    let mut sum = 1;
    for i in 1.. {
        let side = 2 * i + 1;
        if side > 1001 {
            break;
        }
        let s = side * side * 4 - i * 12;
        //println!("side: {}, s: {}", side, s);
        sum += s;
    }
    sum
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 669171001));
}
