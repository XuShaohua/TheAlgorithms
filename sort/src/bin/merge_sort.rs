// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sort::merge_sort;
use sort::util::{is_sorted, read_ints, show_brief};

fn main() {
    let mut list = read_ints();
    println!("[MergeSort] list");
    show_brief(&list);
    merge_sort(&mut list);
    println!("Result:");
    show_brief(&list);
    assert!(is_sorted(&list));
}
