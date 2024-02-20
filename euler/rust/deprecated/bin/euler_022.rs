// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use std::fs::File;
use std::io::{BufReader, Read};

/// Problem:
///
/// Using names.txt (right click and 'Save Link/Target As...'), a 46K
/// text file containing over five-thousand first names, begin by sorting it
/// into alphabetical order. Then working out the alphabetical value for
/// each name, multiply this value by its alphabetical position in the list
/// to obtain a name score.
///
/// For example, when the list is sorted into alphabetical order, COLIN,
/// which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
/// So, COLIN would obtain a score of 938 × 53 = 49714.
///
/// What is the total of all the name scores in the file?

fn method1(words: &[String]) -> u64 {
    let mut scores = 0;
    for (index, word) in words.iter().enumerate() {
        let word_score: u64 = word
            .bytes()
            .filter(|c| c != &b'"')
            .map(|c| (c - b'A' + 1) as u64)
            .sum();

        scores += (index as u64 + 1) * word_score;
    }
    scores
}

fn read_file() -> Option<Vec<String>> {
    if let Some(file_path) = std::env::args_os().nth(1) {
        let file = File::open(file_path).unwrap();
        let mut buffer = BufReader::new(file);
        let mut content = String::new();
        buffer.read_to_string(&mut content).unwrap();
        let mut words: Vec<String> = content.split(',').map(|s| s.to_string()).collect();
        words.sort();
        Some(words)
    } else {
        eprintln!("Usage: {} file_path", std::env::args().next().unwrap());
        None
    }
}

fn main() {
    let words = read_file().unwrap();
    println!("method1: {}", method1(words.as_ref()));
}

#[bench]
fn bench_method1(_b: &mut test::Bencher) {
    // let words = read_file().unwrap();
    // b.iter(|| method1(words.as_ref()));
}
