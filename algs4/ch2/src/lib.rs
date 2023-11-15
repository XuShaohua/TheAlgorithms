// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::path::Path;

mod graph;
pub use graph::export_graph;

mod selection_sort;
pub use selection_sort::selection_sort;

pub fn exchange<T>(list: &mut Vec<T>, i: usize, j: usize)
where
    T: Copy,
{
    let tmp = list[i];
    list[i] = list[j];
    list[j] = tmp;
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
    T: fmt::Display,
{
    for s in vec {
        print!("{} ", s);
    }
    println!();
}
