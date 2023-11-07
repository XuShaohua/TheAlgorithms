// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// Each new term in the Fibonacci sequence is generated by adding the previous
/// two terms. By starting with 1 and 2, the first 10 terms will be:
///               1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
/// By considering the terms in the Fibonacci sequence whose values do not
/// exceed four million, find the sum of the even-valued terms.

fn method1(max_num: usize) -> usize {
    let mut sum = 0;
    let mut prev = 1;
    let mut current = 2;
    let mut tmp;
    let mut odd_num_count = 0;
    while current <= max_num {
        tmp = prev + current;
        prev = current;
        current = tmp;
        odd_num_count += 1;
        if odd_num_count == 1 {
            sum += prev;
        } else if odd_num_count == 3 {
            odd_num_count = 0;
        }
    }

    sum
}

fn method2(max_num: usize) -> usize {
    let mut sum = 0;
    let mut prev = 1;
    let mut current = 2;

    while current < max_num {
        let new = prev + current;
        prev = current;
        current = new;
        if prev % 2 == 0 {
            sum += prev;
        }
    }
    sum
}

fn main() {
    let max_num = 4_000_000;
    println!("sum: {}", method1(max_num));
    println!("sum: {}", method2(max_num));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(4_000_000), 4_613_732));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(4_000_000), 4_613_732));
}