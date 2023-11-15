// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{self, BufRead, BufReader};

pub fn read_strings() -> Vec<String> {
    let mut v = vec![];
    let buffer = BufReader::new(io::stdin());
    let input_iter = buffer.lines();
    for line in input_iter {
        if let Ok(line) = line {
            for word in line.trim().split_whitespace() {
                v.push(word.to_string());
            }
        }
    }
    v
}

pub fn exch(list: &mut Vec<String>, i: usize, j: usize) {
    let mut tmp = String::new();
    std::mem::swap(&mut list[i], &mut tmp);
    std::mem::swap(&mut list[j], &mut tmp);
    std::mem::swap(&mut list[i], &mut tmp);
}

pub fn show(vec: &Vec<String>) {
    for s in vec {
        println!("{}", s);
    }
}
