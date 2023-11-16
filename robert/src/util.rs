// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::io::{self, BufRead, BufReader};

pub fn is_sorted<T>(list: &[T]) -> bool
where
    T: PartialOrd + fmt::Debug,
{
    for i in 0..(list.len() - 1) {
        if list[i] > list[i + 1] {
            println!(
                "Order error at: {i}, values: ({:?}, {:?})",
                list[i],
                list[i + 1]
            );
            return false;
        }
    }
    return true;
}

pub fn show<T>(slice: &[T])
where
    T: fmt::Display,
{
    for s in slice {
        print!("{} ", s);
    }
    println!();
}

/// Read integers from stdin.
///
/// # Panics
/// Raise panic if invalid integer found.
pub fn read_ints() -> Vec<i32> {
    let mut v = vec![];
    let buffer = BufReader::new(io::stdin());
    let input_iter = buffer.lines();
    for line in input_iter.flatten() {
        for word in line.split_whitespace() {
            let value = word.parse::<i32>().expect("Invalid integer");
            v.push(value);
        }
    }
    v
}

#[must_use]
pub fn read_strings() -> Vec<String> {
    let buffer = BufReader::new(io::stdin());
    let input_iter = buffer.lines();
    let mut v = vec![];
    for line in input_iter.flatten() {
        for word in line.split_whitespace() {
            v.push(word.to_string());
        }
    }
    v
}
