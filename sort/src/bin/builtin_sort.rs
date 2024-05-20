// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sort::util::{is_sorted, read_ints, show_brief};

fn main() {
    let mut list = read_ints();
    println!("[Sort with slice.sort()] LIST:");
    show_brief(&list);
    list.sort();
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);
}
