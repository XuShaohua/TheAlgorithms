// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use base::ints;

fn main() {
    let mut list = ints::read_ints();
    sort::selection_sort(&mut list);
    println!("RESULT:");
    sort::generics::show(&list);
}
