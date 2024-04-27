// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Binary search
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    assert!(!nums.is_empty());

    let mut low = 0;
    let mut high = nums.len() - 1;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => low = middle,
            Ordering::Equal => return middle as i32,
            Ordering::Greater => high = middle,
        }
    }
    if nums[high] < target {
        high as i32 + 1
    } else if nums[low] > target {
        low as i32 - 1
    } else {
        high as i32
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let pos = search_insert(nums, target);
    assert_eq!(pos, 2);

    let nums = vec![1, 3, 5, 6];
    let target = 2;
    let pos = search_insert(nums, target);
    assert_eq!(pos, 1);

    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let pos = search_insert(nums, target);
    assert_eq!(pos, 4);
}
