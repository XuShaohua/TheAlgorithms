// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let k = k as usize;

    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if nums[i] == nums[j] && j - i <= k {
                return true;
            }
        }
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    assert!(contains_nearby_duplicate(nums, k));

    let nums = vec![1, 0, 1, 1];
    let k = 1;
    assert!(contains_nearby_duplicate(nums, k));

    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;
    assert!(!contains_nearby_duplicate(nums, k));
}
