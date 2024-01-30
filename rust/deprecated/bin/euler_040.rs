// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// An irrational decimal fraction is created by
/// concatenating the positive integers:
///
/// 0.123456789101112131415161718192021...
///
/// It can be seen that the 12th digit of the fractional part is 1.
///
/// If dn represents the nth digit of the fractional part,
/// find the value of the following expression.
///
/// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

fn method1() -> u64 {
    let mut result = String::with_capacity(1_000_000);
    for i in 1..200_000 {
        let s = i.to_string();
        result += &s;
    }

    let mut product = 1;
    for i in 0..6 {
        let pos = 10_usize.pow(i) - 1;
        product *= &result[pos..pos + 1].parse().unwrap();
    }
    product
}

fn method2() -> u64 {
    const DIGITS: [u32; 6] = [9, 90 * 2, 900 * 3, 9000 * 4, 90_000 * 5, 900_000 * 6];

    let get_digits = |num: u32| {
        let mut n = num;
        let mut i = 0;
        let mut base = 1;
        let mut num_digits = 1;
        while n > DIGITS[i] {
            n -= DIGITS[i];
            i += 1;
            base *= 10;
            num_digits += 1;
        }

        let num_order = (n - 1) / num_digits + 1;
        let digit_order = (n - 1) % num_digits;
        let number = base + num_order - 1;
        let s: String = number.to_string();
        (s.bytes().nth(digit_order as usize).unwrap() - b'0') as u64
    };

    get_digits(1)
        * get_digits(10)
        * get_digits(100)
        * get_digits(1000)
        * get_digits(10_000)
        * get_digits(100_000)
        * get_digits(1_000_000)
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 210));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 210));
}
