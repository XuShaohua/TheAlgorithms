// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

#[allow(clippy::comparison_chain)]
unsafe fn guess(pick: i32) -> i32 {
    static PICKED: i32 = 4;
    if PICKED < pick {
        -1
    } else if PICKED > pick {
        1
    } else {
        0
    }
}

// Binary Search
unsafe fn guess_number(n: i32) -> i32 {
    assert!(n > 0);
    let mut left = 0;
    let mut right = n;
    while left <= right {
        let mid = left + (right - left) / 2;
        let pick = unsafe { guess(mid) };
        if pick == -1 {
            right = mid.saturating_sub(1);
        } else if pick == 1 {
            left = mid + 1;
        } else {
            return mid;
        }
    }
    0
}

fn main() {}
