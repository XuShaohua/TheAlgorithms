// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use base::ints;

use ch2::selection_sort;

fn main() {
    let mut list = ints::read_ints();
    selection_sort(&mut list, true);
}
