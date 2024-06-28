// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
    true
}

pub fn show<T>(slice: &[T])
where
    T: fmt::Display,
{
    for s in slice {
        print!("{s} ");
    }
    println!();
}

pub fn show_brief<T>(slice: &[T])
where
    T: fmt::Display,
{
    if slice.len() < 128 {
        show(slice);
    } else {
        show(&slice[..128]);
        println!("...\n...");
    }
}

/// Read integers from stdin.
///
/// # Panics
/// Raise panic if invalid integer found.
#[must_use]
pub fn read_ints() -> Vec<i32> {
    let mut v = vec![];
    let buffer = BufReader::new(io::stdin());
    let mut input_iter = buffer.lines();
    while let Some(Ok(line)) = input_iter.next() {
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
    let mut input_iter = buffer.lines();
    let mut v = vec![];
    while let Some(Ok(line)) = input_iter.next() {
        for word in line.split_whitespace() {
            v.push(word.to_owned());
        }
    }
    v
}

/// Generate random integers.
///
/// # Errors
/// Returns error if failed to rand system random file.
pub fn random_ints(len: usize) -> Result<Vec<i32>, io::Error> {
    let mut file = File::open("/dev/urandom")?;
    let mut buf = vec![0; len * 4];
    file.read_exact(&mut buf)?;

    let mut nums = vec![0; len];
    for i in 0..len {
        let array: [u8; 4] = [buf[4 * i], buf[4 * i + 1], buf[4 * i + 2], buf[4 * i + 3]];
        nums[i] = i32::from_le_bytes(array);
    }
    Ok(nums)
}

#[cfg(test)]
mod tests {
    use crate::util::random_ints;

    #[test]
    fn test_random_ints() {
        let len = 100;
        let nums: Vec<i32> = random_ints(len).unwrap();
        assert_eq!(nums.len(), len);
    }
}