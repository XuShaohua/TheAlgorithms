// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sort::selection_sort::recursive_selection_sort;
use sort::util::{is_sorted, read_ints, show_brief};

fn main() {
    let mut list = read_ints();
    println!("[RecursiveSelectionSort] LIST:");
    show_brief(&list);
    recursive_selection_sort(&mut list);
    assert!(is_sorted(&list));
    println!("RESULT:");
    show_brief(&list);
}
