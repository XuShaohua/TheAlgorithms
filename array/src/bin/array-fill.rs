// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::sync::atomic::{AtomicI32, Ordering};

fn get_next_id() -> i32 {
    static NEXT_ID: AtomicI32 = AtomicI32::new(1);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

fn main() {
    let mut numbers = [1, 1, 2, 3, 5];
    numbers.fill(0);
    assert_eq!(numbers, [0, 0, 0, 0, 0]);
    numbers.fill_with(|| get_next_id().pow(2));
    assert_eq!(numbers, [1, 4, 9, 16, 25]);
}