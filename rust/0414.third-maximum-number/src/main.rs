// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums.dedup();
    if nums.len() < 3 {
        nums[nums.len() - 1]
    } else {
        nums[nums.len() - 3]
    }
}

fn check_solution() {
    let nums = vec![3, 2, 1];
    assert_eq!(third_max(nums), 1);

    let nums = vec![1, 2];
    assert_eq!(third_max(nums), 2);

    let nums = vec![2, 2, 3, 1];
    assert_eq!(third_max(nums), 1);

    let nums = vec![1, 1, 2];
    assert_eq!(third_max(nums), 2);
}

fn main() {
    check_solution();
}
