// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// If p is the perimeter of a right angle triangle with integral length sides,
/// {a,b,c}, there are exactly three solutions for p = 120.
///
///     {20,48,52}, {24,45,51}, {30,40,50}
///
/// For which value of p ≤ 1000, is the number of solutions maximised?

fn method1() -> u32 {
    let parse_triangle = |num: u32| {
        let one_third = num / 3;
        let half = num / 2;
        let mut count = 0;

        for a in 1..one_third {
            for b in a..half {
                let c = num - a - b;
                if a * a + b * b == c * c {
                    count += 1;
                }
            }
        }

        count
    };

    let mut max_count = 0;
    let mut max_num = 0;
    for i in 120..=1000 {
        let count = parse_triangle(i);
        if count > max_count {
            max_count = count;
            max_num = i;
        }
    }

    max_num
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 840));
}
