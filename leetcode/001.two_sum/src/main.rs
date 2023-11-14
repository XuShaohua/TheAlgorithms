// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [two-sum](https://leetcode.com/problems/two-sum)

fn solution1(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for (i, m) in nums.iter().enumerate() {
        for (j, n) in nums.iter().enumerate() {
            if i != j && m + n == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let nums = &[2, 7, 11, 15];
    let target = 9;
    assert_eq!(solution1(nums, target), Some((0, 1)));

    let nums = &[3, 2, 4];
    let target = 6;
    assert_eq!(solution1(nums, target), Some((1, 2)));

    let nums = &[3, 3];
    let target = 6;
    assert_eq!(solution1(nums, target), Some((0, 1)));
}
