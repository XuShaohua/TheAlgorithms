// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Binary search
pub fn search_insert1(nums: Vec<i32>, target: i32) -> i32 {
    // 先检查边界情况.
    if nums.is_empty() || target <= nums[0] {
        return 0;
    }
    let last = nums.len() - 1;
    if target > nums[last] {
        return nums.len() as i32;
    }
    if target == nums[last] {
        return last as i32;
    }

    // 左闭右闭区间
    let mut left: usize = 1;
    let mut right: usize = last;

    // 终止循环的条件是 nums[left] > nums[right].
    // 此时 left 所在位置就是 target 插入到数组中的位置.
    while left <= right {
        let middle = left + (right - left) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => left = middle + 1,
            Ordering::Equal => return middle as i32,
            // 这里 middle - 1 并不会出现整数 underflow
            Ordering::Greater => right = middle - 1,
        }
    }

    left as i32
}

// 使用 slice::binary_search()
pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(index) => index as i32,
        Err(index) => index as i32,
    }
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let pos = func(nums, target);
    assert_eq!(pos, 2);

    let nums = vec![1, 3, 5, 6];
    let target = 2;
    let pos = func(nums, target);
    assert_eq!(pos, 1);

    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let pos = func(nums, target);
    assert_eq!(pos, 4);

    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let pos = func(nums, target);
    assert_eq!(pos, 0);
}

fn main() {
    check_solution(search_insert1);
    check_solution(search_insert2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, search_insert1, search_insert2};

    #[test]
    fn test_search_insert1() {
        check_solution(search_insert1);
    }

    #[test]
    fn test_search_insert2() {
        check_solution(search_insert2);
    }
}
