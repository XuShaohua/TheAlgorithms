// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    assert!(!nums.is_empty());
    let len = nums.len();
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }
    false
}

pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
    assert!(!nums.is_empty());
    let mut nums = nums;
    nums.sort();
    let len = nums.len();
    for i in 0..(len - 1) {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    assert!(contains_duplicate(nums));

    let nums = vec![1, 2, 3, 4];
    assert!(!contains_duplicate(nums));
    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert!(contains_duplicate(nums));

    let nums = vec![1, 2, 3, 1];
    assert!(contains_duplicate2(nums));
    let nums = vec![1, 2, 3, 4];
    assert!(!contains_duplicate2(nums));
    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert!(contains_duplicate2(nums));
}
