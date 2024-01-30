// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
    let len = nums.len();
    assert!(len > 1);
    let index_diff = index_diff as usize;

    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if (nums[i] - nums[j]).abs() <= value_diff && j - i <= index_diff {
                return true;
            }
        }
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let index_diff = 3;
    let value_diff = 0;
    assert!(contains_nearby_almost_duplicate(
        nums, index_diff, value_diff
    ));

    let nums = vec![1, 5, 9, 1, 5, 9];
    let index_diff = 2;
    let value_diff = 3;
    assert!(!contains_nearby_almost_duplicate(
        nums, index_diff, value_diff
    ));
}
