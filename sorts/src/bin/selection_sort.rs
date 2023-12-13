// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sorts::selection_sort;
use sorts::util::{is_sorted, read_ints, show_brief};

fn main() {
    let mut list = read_ints();
    selection_sort(&mut list);
    assert!(is_sorted(&list));
    println!("RESULT:");
    show_brief(&list);
}
