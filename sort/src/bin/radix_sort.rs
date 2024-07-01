// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sort::radix_sort::radix_sort;
use sort::util::{is_sorted, read_ints, show_brief};

fn main() {
    let list: Vec<i32> = read_ints();
    let mut list: Vec<u32> = list.into_iter().map(|num| num as u32).collect();
    println!("[RadixSort] LIST:");
    show_brief(&list);
    radix_sort(&mut list);
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);
}
