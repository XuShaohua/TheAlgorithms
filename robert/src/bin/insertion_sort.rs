// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use robert::util::{is_sorted, read_ints, show_brief};
use sorts::{insertion_sort, insertion_sort_vanilla};

fn main() {
    let mut list = read_ints();
    println!("[InsertionSort] LIST:");
    show_brief(&list);
    let mut list2 = list.clone();
    insertion_sort(&mut list);
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);

    insertion_sort_vanilla(&mut list2);
    assert!(is_sorted(&list2));
}
