// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Problem:
///

/// The nth term of the sequence of triangle numbers is given by, t_n = ½n(n+1);
/// so the first ten triangle numbers are:
///
///     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// By converting each letter in a word to a number corresponding to
/// its alphabetical position and adding these values we form a word value.
/// For example, the word value for SKY is 19 + 11 + 25 = 55 = t10.
/// If the word value is a triangle number then we shall call the word
/// a triangle word.
///
/// Using words.txt (right click and 'Save Link/Target As...'), a 16K
/// text file containing nearly two-thousand common English words,
/// how many are triangle words?

fn method1(words: &Vec<Vec<u8>>) -> usize {
    let get_word_sum = |v: &Vec<u8>| -> u16 {
        v.iter()
            .map(|c| {
                if c < &b'A' || c > &b'Z' {
                    0
                } else {
                    (c - b'A' + 1) as u16
                }
            })
            .sum()
    };

    let triangle_nums: Vec<u16> = (1..100).map(|i| i * (i + 1) / 2).collect();
    let is_triangle_num = |n: u16| -> bool { triangle_nums.binary_search(&n).is_ok() };

    words
        .iter()
        .map(|word| get_word_sum(word))
        .filter(|n| is_triangle_num(*n))
        .count()
}

fn read_words(filepath: &str) -> Result<Vec<Vec<u8>>> {
    let fd = File::open(filepath)?;
    let buf = BufReader::new(fd);
    Ok(buf.split(b',').map(|l| l.unwrap()).collect())
}

fn main() {
    let filepath = if let Some(filepath) = std::env::args().nth(1) {
        filepath
    } else {
        panic!("Usage: euler_042 words.txt");
    };
    let words = read_words(&filepath).unwrap();
    println!("method1: {}", method1(&words));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    let words = read_words("/etc/issue").unwrap();
    b.iter(|| method1(&words));
}
