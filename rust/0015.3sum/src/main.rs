// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Solve with two-pointers method.
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut result = Vec::new();

    let len = nums.len();

    for i in 0..(len - 2) {
        let target = -nums[i];

        if target < 0 {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = len - 1;

        while left < right {
            let sum = nums[left] + nums[right];
            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
                // Remove duplicates
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            }
        }
    }

    result
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let out = three_sum(nums);
    assert_eq!(out, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    let nums = vec![0, 1, 1];
    let out = three_sum(nums);
    assert!(out.is_empty());

    let nums = vec![0, 0, 0];
    let out = three_sum(nums);
    assert_eq!(out, vec![vec![0, 0, 0]]);
}
