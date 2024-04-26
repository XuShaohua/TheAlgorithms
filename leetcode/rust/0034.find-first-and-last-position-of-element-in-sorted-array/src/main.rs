// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Binary search
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    let mut result = vec![-1, -1];
    if nums.is_empty() {
        return result;
    }

    let len = len as i32;
    let mut low = 0;
    let mut high = len - 1;
    let mut middle: i32 = 0;
    while low <= high {
        middle = low + (high - low) / 2;
        match nums[middle as usize].cmp(&target) {
            Ordering::Less => low = middle + 1,
            Ordering::Greater => high = middle - 1,
            Ordering::Equal => break,
        }
    }

    let mut i = middle;
    while i >= 0 && nums[i as usize] == target {
        result[0] = i;
        i -= 1;
    }
    let mut i = middle;
    while i < len && nums[i as usize] == target {
        result[1] = i;
        i += 1;
    }
    result
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

    let nums = vec![1];
    let target = 1;
    assert_eq!(search_range(nums, target), [0, 0]);

    let nums = vec![1];
    let target = 0;
    assert_eq!(search_range(nums, target), [-1, -1]);

    let nums = vec![2, 2];
    let target = 2;
    assert_eq!(search_range(nums, target), [0, 1]);
}
