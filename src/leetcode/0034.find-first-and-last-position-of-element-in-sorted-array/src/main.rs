// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Binary search
pub fn search_range1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![-1, -1];
    let len = nums.len();

    // 处理极端情况
    if len == 0 || nums[0] > target || nums[len - 1] < target {
        return result;
    }

    let mut low = 0;
    let mut high = len - 1;
    let mut middle = 0;
    while low <= high {
        middle = low + (high - low) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => low = middle + 1,
            Ordering::Equal => break,
            Ordering::Greater => {
                if middle > 1 {
                    high = middle - 1;
                } else {
                    // 没有找到
                    return result;
                }
            }
        }
    }

    // 退化成线性查找
    let mut i = middle as i32;
    while i >= 0 && nums[i as usize] == target {
        result[0] = i;
        i -= 1;
    }
    let mut i = middle;
    while i < len && nums[i] == target {
        result[1] = i as i32;
        i += 1;
    }
    result
}

// 使用 slice::binary_search()
pub fn search_range2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![-1, -1];
    let len = nums.len();

    // 处理极端情况
    if len == 0 || nums[0] > target || nums[len - 1] < target {
        return result;
    }

    let middle = match nums.binary_search(&target) {
        Ok(index) => index,
        Err(_) => return result,
    };

    // 退化成线性查找
    let mut i = middle as i32;
    while i >= 0 && nums[i as usize] == target {
        result[0] = i;
        i -= 1;
    }
    let mut i = middle;
    while i < len && nums[i] == target {
        result[1] = i as i32;
        i += 1;
    }
    result
}

pub type SolutionFn = fn(Vec<i32>, i32) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    assert_eq!(func(nums, target), [3, 4]);

    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;
    assert_eq!(func(nums, target), [-1, -1]);

    let nums = vec![];
    let target = 0;
    assert_eq!(func(nums, target), [-1, -1]);

    let nums = vec![1];
    let target = 1;
    assert_eq!(func(nums, target), [0, 0]);

    let nums = vec![1];
    let target = 0;
    assert_eq!(func(nums, target), [-1, -1]);

    let nums = vec![2, 2];
    let target = 2;
    assert_eq!(func(nums, target), [0, 1]);
}

fn main() {
    check_solution(search_range1);
    check_solution(search_range2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, search_range1, search_range2};

    #[test]
    fn test_search_range1() {
        check_solution(search_range1);
    }

    #[test]
    fn test_search_range2() {
        check_solution(search_range2);
    }
}
