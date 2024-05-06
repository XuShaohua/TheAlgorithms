// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Binary Search
// 直接法, 找到元素后就直接返回.
pub fn search1(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() || nums[0] > target || nums[nums.len() - 1] < target {
        return -1;
    }

    // 左闭右闭区间
    let mut low = 0;
    let mut high = nums.len() - 1;

    // 退出循环的条件是 left > right.
    while low <= high {
        // 防止整数平均值溢出.
        let middle = low + (high - low) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => low = middle + 1,
            Ordering::Equal => return middle as i32,
            Ordering::Greater => {
                if middle < 1 {
                    return -1;
                } else {
                    high = middle - 1;
                }
            }
        }
    }
    -1
}

// Binary Search
// 排除法
pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() || nums[0] > target || nums[nums.len() - 1] < target {
        return -1;
    }

    // 左闭右开区间
    let mut left = 0;
    let mut right = nums.len();

    // 终止循环的条件是 left == right
    while left < right {
        // 中间节点的计算不一样.
        let mid = left + (right - left) / 2;
        // 排除 [left, mid] 区间, 在 [mid + 1, right) 区间内查找
        if nums[mid] < target {
            left = mid + 1;
        } else {
            // 排除 [mid, right] 区间, 在 [left, mid) 区间内查找
            right = mid;
        }
    }

    // 检查剩余空间内的元素, nums[left] == nums[right] == target
    if left < nums.len() && nums[left] == target {
        left as i32
    } else {
        -1
    }
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(func(nums, target), 4);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    assert_eq!(func(nums, target), -1);

    let nums = vec![];
    let target = 2;
    assert_eq!(func(nums, target), -1);

    let nums = vec![5];
    let target = -5;
    assert_eq!(func(nums, target), -1);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 13;
    assert_eq!(func(nums, target), -1);
}

fn main() {
    check_solution(search1);
    check_solution(search2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, search1, search2};

    #[test]
    fn test_search1() {
        check_solution(search1);
    }

    #[test]
    fn test_search2() {
        check_solution(search2);
    }
}
