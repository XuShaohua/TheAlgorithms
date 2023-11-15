// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{self, BufRead, BufReader};

pub fn read_ints() -> Vec<i32> {
    let mut v = vec![];
    let buffer = BufReader::new(io::stdin());
    let input_iter = buffer.lines();
    for line in input_iter {
        if let Ok(line) = line {
            for word in line.trim().split_whitespace() {
                let value = word.parse::<i32>().expect("Invalid integer");
                v.push(value);
            }
        }
    }
    v
}
