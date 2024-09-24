// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Brute force
pub fn three_sum_closest1(nums: Vec<i32>, target: i32) -> i32 {
    assert!(nums.len() >= 3);
    let mut closest = nums[0] + nums[1] + nums[2];
    let len = nums.len();
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            for k in (j + 1)..len {
                let sum = nums[i] + nums[j] + nums[k];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
            }
        }
    }
    closest
}

// 靠拢型双指针
pub fn three_sum_closest2(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    assert!(nums.len() >= 3);

    // 对数组排序.
    let mut nums = nums;
    nums.sort();

    // 先处理特别情况.
    let mut closest: i32 = nums[0] + nums[1] + nums[2];
    if len == 3 || closest == target {
        return closest;
    }
    let mut closest_diff: i32 = i32::MAX;

    // 遍历数组.
    for i in 0..(len - 2) {
        // 初始化双指针.
        let mut left = i + 1;
        let mut right = len - 1;
        while left < right {
            // 不需要检查整数溢出.
            let sum = nums[i] + nums[left] + nums[right];
            match sum.cmp(&target) {
                // 如果找到了与 `target` 相同的结果, 就不需要再循环了, 直接返回.
                Ordering::Equal => return sum,
                // 移动指针
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }

            let diff = (sum - target).abs();
            if diff < closest_diff {
                // 更新新的最接近值.
                closest = sum;
                closest_diff = diff;
            }
        }
    }

    closest
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    assert_eq!(func(nums, target), 2);

    let nums = vec![0, 0, 0];
    let target = 1;
    assert_eq!(func(nums, target), 0);
}

fn main() {
    check_solution(three_sum_closest1);
    check_solution(three_sum_closest2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, three_sum_closest1};

    #[test]
    fn test_three_sum_closest1() {
        check_solution(three_sum_closest1);
    }
}
