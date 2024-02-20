// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use std::collections::HashMap;

/// Problem:
///
/// If the numbers 1 to 5 are written out in words: one, two, three, four,
/// five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
/// If all the numbers from 1 to 1000 (one thousand) inclusive were
/// written out in words, how many letters would be used?
///
/// NOTE: Do not count spaces or hyphens. For example, 342
/// (three hundred and forty-two) contains 23 letters and
/// 115 (one hundred and fifteen) contains 20 letters. The use of "and"
/// when writing out numbers is in compliance with British usage.

const LETTERS: [(u32, &str); 27] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (30, "thirty"),
    (40, "forty"),
    (50, "fifty"),
    (60, "sixty"),
    (70, "seventy"),
    (80, "eighty"),
    (90, "ninety"),
];

fn method1() -> usize {
    let mut letters = HashMap::new();
    for letter in &LETTERS {
        letters.insert(letter.0, letter.1.to_string());
    }

    let get_letters = |num: u32| -> usize {
        let mut sum = Vec::new();
        let mut n = num;
        if n >= 1000 {
            let quotient = n / 1000;
            n %= 1000;
            if let Some(l) = letters.get(&quotient) {
                sum.push(l.clone());
            } else {
                panic!("letters not contain: {}", quotient);
            }
            sum.push("thousand".to_string());
        }
        if n >= 100 {
            let quotient = n / 100;
            n %= 100;

            if let Some(l) = letters.get(&quotient) {
                sum.push(l.clone());
            } else {
                panic!("letters not contain: {}", quotient);
            }
            sum.push("hundred".to_string());
            if n > 0 {
                sum.push("and".to_string());
            }
        }
        if n >= 20 {
            let quotient = n / 10 * 10;
            if let Some(l) = letters.get(&quotient) {
                sum.push(l.clone());
            } else {
                panic!("letters not contain: {}", quotient);
            }
            n %= 10;
        } else if n >= 10 {
            if let Some(l) = letters.get(&n) {
                sum.push(l.clone());
            } else {
                panic!("letters not contain: {}", n);
            }
        }

        if n > 0 && n < 10 {
            if let Some(l) = letters.get(&n) {
                sum.push(l.clone());
            } else {
                panic!("letters not contain: {}", n);
            }
        }

        sum.iter().map(|word| word.len()).sum()
    };

    (1..=1000).map(get_letters).sum()
}

fn method2() -> usize {
    let mut letters = HashMap::new();
    for letter in &LETTERS {
        letters.insert(letter.0, letter.1.len());
    }

    let get_letters = |num: u32| -> usize {
        let mut sum = 0;
        let mut n = num;
        if n >= 1000 {
            let quotient = n / 1000;
            n %= 1000;
            if let Some(l) = letters.get(&quotient) {
                sum += l;
            } else {
                panic!("letters not contain: {}", quotient);
            }
            sum += "thousand".len();
        }
        if n >= 100 {
            let quotient = n / 100;
            n %= 100;

            if let Some(l) = letters.get(&quotient) {
                sum += l;
            } else {
                panic!("letters not contain: {}", quotient);
            }
            sum += "hundred".len();
            if n > 0 {
                sum += "and".len();
            }
        }
        if n >= 20 {
            let quotient = n / 10 * 10;
            if let Some(l) = letters.get(&quotient) {
                sum += l;
            } else {
                panic!("letters not contain: {}", quotient);
            }
            n %= 10;
        } else if n >= 10 {
            if let Some(l) = letters.get(&n) {
                sum += l;
            } else {
                panic!("letters not contain: {}", n);
            }
        }

        if n > 0 && n < 10 {
            if let Some(l) = letters.get(&n) {
                sum += l;
            } else {
                panic!("letters not contain: {}", n);
            }
        }
        sum
    };

    (1..=1000).map(get_letters).sum()
}

fn method3() -> usize {
    let mut letters = HashMap::new();
    for letter in &LETTERS {
        letters.insert(letter.0, letter.1.len());
    }

    let get_letters = |num: u32| -> usize {
        let mut sum = 0;
        let mut n = num;
        let mut quotient;
        if n >= 1000 {
            quotient = n / 1000;
            sum += letters[&quotient];
            sum += "thousand".len();
            n %= 1000;
        }
        if n >= 100 {
            quotient = n / 100;
            sum += letters[&quotient];
            sum += "hundred".len();

            n %= 100;
            if n > 0 {
                sum += "and".len();
            }
        }
        if n >= 20 {
            quotient = n / 10 * 10;
            sum += letters[&quotient];
            n %= 10;
        } else if n >= 10 {
            sum += letters[&n];
        }

        if n > 0 && n < 10 {
            sum += letters[&n];
        }
        sum
    };

    let mut sum = 0;
    for i in 1..=1000 {
        sum += get_letters(i);
    }
    sum
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
    println!("method3: {}", method3());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 21124));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(), 21124));
}

#[bench]
fn bench_method3(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method3(), 21124));
}
