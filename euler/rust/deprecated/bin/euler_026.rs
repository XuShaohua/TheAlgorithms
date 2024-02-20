// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// A unit fraction contains 1 in the numerator. The decimal
/// representation of the unit fractions with denominators 2 to 10
/// are given:
///
///  1/2 = 0.5
///  1/3 = 0.(3)
///  1/4 = 0.25
///  1/5 = 0.2
///  1/6 = 0.1(6)
///  1/7 = 0.(142857)
///  1/8 = 0.125
///  1/9 = 0.(1)
///  1/10 = 0.1
///
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle.
/// It can be seen that 1/7 has a 6-digit recurring cycle.
///
/// Find the value of d < 1000 for which 1/d contains the longest
/// recurring cycle in its decimal fraction part.

fn method1() -> u32 {
    let mut longest_cycle = 1;
    let mut longest_cycle_num = 0;

    let mut buf = vec![];
    let mut get_cycle = |i: u32| -> usize {
        buf.clear();
        let mut num = 1;
        loop {
            let r = (num % i) * 10;
            if r == 0 {
                return 0;
            }
            if let Some(index) = buf.iter().position(|n| n == &r) {
                return buf.len() - index;
            }
            buf.push(r);
            num = r;
        }
    };

    for i in 2..1000 {
        let cycle = get_cycle(i);
        if cycle > longest_cycle {
            //println!("> {} => {}, longest: {}", i, cycle, longest_cycle);
            longest_cycle = cycle;
            longest_cycle_num = i;
        }
    }

    longest_cycle_num
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 983));
}
