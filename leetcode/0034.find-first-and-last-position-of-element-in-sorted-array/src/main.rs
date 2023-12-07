// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Binary search
pub fn search_range(nums: Vec<i32>, target: i32) -> [i32; 2] {
    const NOT_FOUND: [i32; 2] = [-1, -1];

    let len = nums.len();
    if nums.is_empty() || nums[0] > target || nums[len - 1] < target {
        return NOT_FOUND;
    }

    let mut low = 0;
    let mut high = len - 1;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => low = middle,
            Ordering::Greater => high = middle,
            Ordering::Equal => break,
        }
    }
    if low + 1 == high {
        return NOT_FOUND;
    }

    while nums[low] == target {
        low -= 1;
    }
    while nums[high] == target {
        high += 1;
    }
    [(low + 1) as i32, (high - 1) as i32]
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    assert_eq!(search_range(nums, target), [3, 4]);

    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;
    assert_eq!(search_range(nums, target), [-1, -1]);

    let nums = vec![];
    let target = 0;
    assert_eq!(search_range(nums, target), [-1, -1]);
}
