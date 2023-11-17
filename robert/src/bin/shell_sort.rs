// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use robert::util::{is_sorted, read_ints, show_brief};
use sorts::shell_sort;

fn main() {
    let mut list = read_ints();
    println!("[ShellSort] LIST:");
    show_brief(&list);
    shell_sort(&mut list);
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);
}
