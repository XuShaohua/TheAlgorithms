// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// It can be seen that the number, 125874, and its double, 251748,
/// contain exactly the same digits, but in a different order.
///
/// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x,
/// contain the same digits.

fn method1() -> u32 {
    let get_digits = |num: u32| {
        let mut n = num;
        let mut digits: [u8; 8] = [0; 8];
        let mut i = 0;
        while n > 9 {
            digits[i] = (n % 10) as u8;
            i += 1;
            n /= 10;
        }
        digits[i] = n as u8;
        digits.sort();
        digits
    };

    for i in 100.. {
        let digits = get_digits(i);
        if digits == get_digits(i * 2)
            && digits == get_digits(i * 3)
            && digits == get_digits(i * 4)
            && digits == get_digits(i * 5)
            && digits == get_digits(i * 6)
        {
            return i;
        }
    }
    0
}

fn method2() -> u32 {
    let get_digits = |num: u32| {
        let mut n = num;
        let mut digits: [u8; 8] = [0; 8];
        let mut i = 0;
        while n > 9 {
            digits[i] = (n % 10) as u8;
            i += 1;
            n /= 10;
        }
        digits[i] = n as u8;
        digits.sort();
        digits
    };

    let get_base = |num: u32| {
        let mut n = num;
        let mut base = 1;
        while n > 9 {
            n /= 10;
            base += 1;
        }
        base
    };

    for i in 100.. {
        let num = i + 10_u32.pow(get_base(i));
        let digits = get_digits(num);
        if digits == get_digits(num * 2)
            && digits == get_digits(num * 3)
            && digits == get_digits(num * 4)
            && digits == get_digits(num * 5)
            && digits == get_digits(num * 6)
        {
            return num;
        }
    }
    0
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 142857));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 142857));
}
