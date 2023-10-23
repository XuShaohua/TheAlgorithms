// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// A permutation is an ordered arrangement of objects. For example,
/// 3124 is one possible permutation of the digits 1, 2, 3 and 4.
/// If all of the permutations are listed numerically or alphabetically,
/// we call it lexicographic order. The lexicographic permutations of
/// 0, 1 and 2 are:
///
///     012   021   102   120   201   210
///
/// What is the millionth lexicographic permutation of the digits
/// 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn method1() -> u64 {
    // FIXME(Shaohua): Move example code to here
    2783915460
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 2783915460));
}
