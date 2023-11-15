// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use robert::sorting::selection_sort;
use robert::util::{read_ints, show};

fn main() {
    let mut list = read_ints();
    selection_sort(&mut list);
    println!("RESULT:");
    show(&list);
}
