// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5,
/// we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.

fn method1(max_num: usize) -> usize {
    let mut arr = vec![false; max_num + 1];
    for i in 1..max_num {
        let mul = i * 3;
        if mul > max_num {
            break;
        }
        arr[mul] = true;
    }

    for i in 1..max_num {
        let mul = i * 5;
        if mul > max_num {
            break;
        }
        arr[mul] = true;
    }

    let mut sum = 0;
    for i in 1..max_num {
        if arr[i] {
            sum += i;
        }
    }
    sum
}

fn method2(max_num: usize) -> usize {
    let mut sum = 0;
    for i in 1..max_num {
        let mul = i * 3;
        if mul >= max_num {
            break;
        }
        sum += mul;
    }

    let mut reminder = 0;
    for i in 1..max_num {
        let mul = i * 5;
        if mul >= max_num {
            break;
        }
        reminder += 1;
        if reminder == 3 {
            reminder = 0;
        } else {
            sum += mul;
        }
    }
    sum
}

fn method3(max_num: usize) -> usize {
    let mut sum = 0;
    let mut tmp = 0;
    while tmp < max_num {
        sum += tmp;
        tmp += 3;
    }
    tmp = 0;
    while tmp < max_num {
        sum += tmp;
        tmp += 5;
    }
    tmp = 0;
    while tmp < max_num {
        sum -= tmp;
        tmp += 15;
    }
    sum
}

fn main() {
    let max_num = 1000;
    println!("sum in method1: {}", method1(max_num));
    println!("sum in method2: {}", method2(max_num));
    println!("sum in method3: {}", method3(max_num));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(1000), 233_168));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(1000), 233_168));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(1000), 233_168));
}
