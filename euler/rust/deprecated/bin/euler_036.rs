// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// The decimal number, 585 = 10010010012 (binary), is palindromic
/// in both bases.
///
/// Find the sum of all numbers, less than one million, which are palindromic
/// in base 10 and base 2.
///
/// (Please note that the palindromic number, in either base, may not
/// include leading zeros.)

fn is_palindrome(n: u32, base: u8) -> bool {
    let s: String = if base == 10 {
        n.to_string()
    } else {
        format!("{:b}", n)
    };
    let rev_s: String = s.chars().rev().collect();
    s == rev_s
}

fn method1() -> u32 {
    let mut palindromes: Vec<u32> = Vec::new();

    for i in 1..1_000_000 {
        if is_palindrome(i, 10) && is_palindrome(i, 2) {
            palindromes.push(i);
        }
    }

    palindromes.iter().sum()
}

fn is_palindrome2(num: u32, base: u32) -> bool {
    let mut n = num;
    let mut digits = vec![];
    while n >= base {
        digits.push(n % base);
        n /= base;
    }
    digits.push(n);

    let len = digits.len();
    for i in 0..len {
        if digits[i] != digits[len - i - 1] {
            return false;
        }
    }

    true
}

fn method2() -> u32 {
    let mut palindromes: Vec<u32> = Vec::new();

    for i in 1..1_000_000 {
        if is_palindrome2(i, 10) && is_palindrome2(i, 2) {
            palindromes.push(i);
        }
    }

    palindromes.iter().sum()
}

fn method3() -> u32 {
    // 1_000_000 consumes 20 bits in binary format.
    let mut digits = Vec::with_capacity(20);
    let mut is_palindrome3 = |num: u32, base: u32| -> bool {
        let mut n = num;
        digits.clear();

        while n >= base {
            digits.push(n % base);
            n /= base;
        }
        digits.push(n);

        let len = digits.len();
        for i in 0..len {
            if digits[i] != digits[len - i - 1] {
                return false;
            }
        }

        true
    };

    let mut palindromes: Vec<u32> = Vec::new();

    for i in 1..1_000_000 {
        if is_palindrome3(i, 10) && is_palindrome3(i, 2) {
            palindromes.push(i);
        }
    }

    palindromes.iter().sum()
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
    println!("method3: {}", method3());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 872187));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 872187));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(), 872187));
}
