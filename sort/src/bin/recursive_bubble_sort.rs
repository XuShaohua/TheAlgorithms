// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sort::bubble_sort::recursive_bubble_sort;
use sort::util::{is_sorted, read_ints, show_brief};

fn main() {
    let mut list = read_ints();
    println!("[RecursiveBubbleSort] LIST:");
    show_brief(&list);
    recursive_bubble_sort(&mut list);
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);
}
