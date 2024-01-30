// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// A number chain is created by continuously adding the square of the digits
/// in a number to form a new number until it has been seen before.
///
/// For example,
///
/// 44 → 32 → 13 → 10 → 1 → 1
/// 85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
///
/// Therefore any chain that arrives at 1 or 89 will become stuck in an
/// endless loop. What is most amazing is that EVERY starting number will
/// eventually arrive at 1 or 89.
///
/// How many starting numbers below ten million will arrive at 89?

fn method1() -> u64 {
    let mut count = 0;
    for i in 1..10_000_000_u32 {
        let mut s = get_square_digit(i);
        while s != 89 && s != 1 {
            s = get_square_digit(s);
        }
        if s == 89 {
            count += 1;
        }
    }

    count
}

fn get_square_digit(mut num: u32) -> u32 {
    let mut r;
    let mut sum = 0;
    while num >= 10 {
        r = num % 10;
        num /= 10;
        sum += r * r;
    }
    sum + num * num
}

fn method2() -> u64 {
    let mut cache: [bool; 1000] = [false; 1000];

    let mut count = 0;
    for i in 1..10_000_000_u32 {
        let mut s = get_square_digit(i);
        while s != 89 && s != 1 {
            if cache[s as usize] {
                s = 89;
                break;
            }
            s = get_square_digit(s);
        }
        if s == 89 {
            count += 1;
            if i < 1000 {
                cache[i as usize] = true;
            }
        }
    }

    count
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 8581146));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 8581146));
}
