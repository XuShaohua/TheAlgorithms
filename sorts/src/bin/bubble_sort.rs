// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sorts::bubble_sort;
use sorts::util::{is_sorted, read_ints, show_brief};

fn main() {
    let mut list = read_ints();
    println!("[BubbleSort] LIST:");
    show_brief(&list);
    bubble_sort(&mut list);
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);
}