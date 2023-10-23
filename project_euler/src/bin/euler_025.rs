// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// The Fibonacci sequence is defined by the recurrence relation:
///     Fn = F_(n−1) + F_(n−2), where F1 = 1 and F2 = 1.
/// Hence the first 12 terms will be:
///     F1 = 1
///     F2 = 1
///     F3 = 2
///     F4 = 3
///     F5 = 5
///     F6 = 8
///     F7 = 13
///     F8 = 21
///     F9 = 34
///     F10 = 55
///     F11 = 89
///     F12 = 144
/// The 12th term, F12, is the first term to contain three digits.
/// What is the index of the first term in the Fibonacci sequence to
/// contain 1000 digits?

fn method1() -> u32 {
    const MAX_NUM: usize = 1000;
    let mut a: Vec<u8> = Vec::with_capacity(MAX_NUM);
    let mut b: Vec<u8> = Vec::with_capacity(MAX_NUM);
    let mut digits = 1;
    a.push(1);
    b.push(1);
    let mut index = 2;
    loop {
        index += 1;
        let mut sum = 0;
        let mut add_digit = false;
        for digit in 0..digits {
            sum += a[digit] + b[digit];
            a[digit] = b[digit];
            if sum > 9 {
                b[digit] = sum - 10;
                sum = 1;
                add_digit = true;
            } else {
                b[digit] = sum;
                sum = 0;
                add_digit = false;
            }
        }
        if add_digit {
            b.push(1);
            a.push(0);
            digits += 1;
        }
        if (digits + 1) > MAX_NUM {
            break;
        }
    }
    index
}

fn method2() -> u32 {
    const MAX_NUM: usize = 1000;
    let mut a: Vec<u8> = Vec::with_capacity(MAX_NUM + 1);
    let mut b: Vec<u8> = Vec::with_capacity(MAX_NUM + 1);
    for _i in 0..MAX_NUM {
        a.push(0);
        b.push(0);
    }
    let mut digits = 1;
    a[0] = 1;
    b[0] = 1;
    let mut index = 2;
    loop {
        index += 1;
        let mut sum = 0;
        let mut add_digit = false;
        for digit in 0..digits {
            sum += a[digit] + b[digit];
            a[digit] = b[digit];
            if sum > 9 {
                b[digit] = sum - 10;
                sum = 1;
                add_digit = true;
            } else {
                b[digit] = sum;
                sum = 0;
                add_digit = false;
            }
        }
        if add_digit {
            b[digits] = 1;
            digits += 1;
        }
        if (digits + 1) > MAX_NUM {
            break;
        }
    }
    index
}

fn method3() -> u32 {
    const MAX_NUM: usize = 1000;
    let mut a: [u8; MAX_NUM] = [0; MAX_NUM];
    let mut b: [u8; MAX_NUM] = [0; MAX_NUM];
    let mut digits = 1;
    a[0] = 1;
    b[0] = 1;

    let mut index = 2;
    let mut sum;
    loop {
        index += 1;
        sum = 0;
        let mut add_digit = false;
        for digit in 0..digits {
            sum += a[digit] + b[digit];
            a[digit] = b[digit];
            if sum > 9 {
                b[digit] = sum - 10;
                sum = 1;
                add_digit = true;
            } else {
                b[digit] = sum;
                sum = 0;
                add_digit = false;
            }
        }
        if add_digit {
            b[digits] = 1;
            digits += 1;
        }
        if (digits + 1) > MAX_NUM {
            break;
        }
    }
    index
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
    println!("method3: {}", method3());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 4782));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 4782));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(), 4782));
}
