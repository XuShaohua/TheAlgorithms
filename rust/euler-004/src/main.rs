// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// A palindromic number reads the same both ways. The largest palindrome made
/// from the product of two 2-digit numbers is 9009 = 91 x 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.

fn method1() -> u32 {
    let mut largest_palindrome = 0;
    for i in (1..999).rev() {
        for j in (1..i).rev() {
            let product = i * j;
            if is_palindrome1(product) {
                if product > largest_palindrome {
                    largest_palindrome = product;
                }
                break;
            } else if product < largest_palindrome {
                break;
            }
        }
    }
    largest_palindrome
}

fn method2() -> u32 {
    let mut largest_palindrome = 0;
    for i in (1..999).rev() {
        for j in (1..i).rev() {
            let product = i * j;
            if product < largest_palindrome {
                break;
            } else if is_palindrome2(product) {
                if product > largest_palindrome {
                    largest_palindrome = product;
                }
                break;
            }
        }
    }
    largest_palindrome
}

fn is_palindrome1(num: u32) -> bool {
    let s = num.to_string();
    let rev_s: String = s.chars().rev().collect();
    rev_s == s
}

fn is_palindrome2(num: u32) -> bool {
    let mut mut_num = num;
    let mut rev_num = 0;
    while mut_num > 0 {
        rev_num = rev_num * 10 + (mut_num % 10);
        mut_num /= 10;
    }
    num == rev_num
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 906609));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 906609));
}

#[bench]
fn bench_palindrome1(b: &mut test::Bencher) {
    b.iter(|| {
        for i in (1..999_999).step_by(10) {
            is_palindrome1(i);
        }
    });
}

#[bench]
fn bench_palindrome2(b: &mut test::Bencher) {
    b.iter(|| {
        for i in (1..999_999).step_by(1) {
            is_palindrome2(i);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome1() {
        assert!(is_palindrome1(9009));
        assert!(is_palindrome1(906609));
        assert!(!is_palindrome1(98788));
    }

    #[test]
    fn test_palindrome2() {
        assert!(is_palindrome2(9009));
        assert!(is_palindrome2(906609));
        assert!(!is_palindrome2(98788));
    }
}
