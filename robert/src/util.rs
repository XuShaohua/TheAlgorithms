// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{self, BufRead, BufReader};

pub fn exch<T>(list: &mut Vec<T>, i: usize, j: usize)
where
    T: Clone,
{
    let tmp = list[i].clone();
    list[i] = list[j].clone();
    list[j] = tmp.clone();
}

pub fn is_sorted<T>(list: &[T]) -> bool
where
    T: PartialOrd,
{
    for i in 0..(list.len() - 1) {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn show<T>(vec: &[T])
where
    T: std::fmt::Display,
{
    for s in vec {
        print!("{} ", s);
    }
    println!();
}

#[allow(dead_code)]
struct Generics<T> {
    t: std::marker::PhantomData<T>,
}

impl<T: Copy + std::fmt::Display> Generics<T> {
    #[allow(dead_code)]
    pub fn exch(list: &mut Vec<T>, i: usize, j: usize) {
        let tmp = list[i];
        list[i] = list[j];
        list[j] = tmp;
    }

    #[allow(dead_code)]
    pub fn show(vec: &Vec<T>) {
        for s in vec {
            print!("{} ", s);
        }
        println!("");
    }
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

pub fn exch_strings(list: &mut [String], i: usize, j: usize) {
    let mut tmp = String::new();
    std::mem::swap(&mut list[i], &mut tmp);
    std::mem::swap(&mut list[j], &mut tmp);
    std::mem::swap(&mut list[i], &mut tmp);
}
