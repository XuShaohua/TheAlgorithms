// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
///     a^2 + b^2 = c^2
///
/// For example, 3^2 + 4^2 = 9 + 16 = 2^5 = 52.
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.

fn method1() -> u64 {
    for c in 333..1000 {
        for a in 1..c {
            let b = 1000 - a - c;
            if a * a + b * b == c * c {
                println!(">: {}, {}, {}", a, b, c);
                return a * b * c;
            }
        }
    }
    0
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 31875000));
}
