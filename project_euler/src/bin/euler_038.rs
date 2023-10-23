// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// Take the number 192 and multiply it by each of 1, 2, and 3:
///
///     192 × 1 = 192
///     192 × 2 = 384
///     192 × 3 = 576
///
/// By concatenating each product we get the 1 to 9 pandigital, 192384576.
/// We will call 192384576 the concatenated product of 192 and (1,2,3)
///
/// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4,
/// and 5, giving the pandigital, 918273645, which is the concatenated product
/// of 9 and (1,2,3,4,5).
///
/// What is the largest 1 to 9 pandigital 9-digit number that can be formed
/// as the concatenated product of an integer with (1,2, ... , n) where n > 1?

pub struct Pandigits {
    digits: [bool; Self::MAX],
    num: u64,
}

impl Pandigits {
    const MAX: usize = 10;

    pub fn new() -> Pandigits {
        Pandigits {
            digits: [false; Self::MAX],
            num: 0,
        }
    }

    pub fn reset(&mut self) {
        for i in 0..Self::MAX {
            self.digits[i] = false;
        }
        self.num = 0;
    }

    pub fn append(&mut self, orig_n: u64) -> bool {
        let mut r: usize;
        let mut n = orig_n;
        let mut count = 0;
        while n > 0 {
            r = (n % 10) as usize;
            n /= 10;
            if r == 0 || self.digits[r] {
                return false;
            }
            self.digits[r] = true;
            count += 1;
        }
        self.num = self.num * 10_u64.pow(count) + orig_n;
        true
    }

    pub fn is_pandigitals(&self) -> bool {
        for i in 1..Self::MAX {
            if !self.digits[i] {
                return false;
            }
        }
        true
    }

    pub fn get_num(&self) -> u64 {
        self.num
    }
}

fn method1() -> u64 {
    let mut pandigits = Pandigits::new();
    let mut max_pandigitals = 0;
    let ranges = vec![
        (2, 1000, 9999),
        (3, 1000, 9999),
        (4, 100, 999),
        (5, 1, 99),
        (6, 1, 9),
    ];

    for (n, start, end) in ranges.into_iter() {

        'i_range:
        for i in start..end {
            pandigits.reset();
            for j in 1..=n {
                if !pandigits.append(i * j) {
                    continue 'i_range;
                }
            }

            if pandigits.is_pandigitals() {
                let p = pandigits.get_num();
                println!("> i: {}, p: {}, max: {}", i, p, max_pandigitals);
                if p > max_pandigitals {
                    max_pandigitals = p;
                }
            }
        }
    }

    max_pandigitals
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 932718654));
}
