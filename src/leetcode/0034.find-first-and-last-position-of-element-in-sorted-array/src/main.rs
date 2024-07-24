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

// 两次二查找法
// 如果 `target` 在数组中的个数比较多, 这种算法效率很高, `O(log n)`
pub fn search_range3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn search_left(nums: &[i32], target: i32) -> i32 {
        let len = nums.len();
        // 极端情况
        if len == 0 || nums[0] > target || nums[len - 1] < target {
            return -1;
        }

        // 极端情况
        if nums[0] == target {
            return 0;
        }

        let mut left = 1;
        let mut right = len - 1;
        let mut low: i32 = -1;
        while left <= right {
            let middle = left + (right - left) / 2;
            match nums[middle].cmp(&target) {
                Ordering::Less => left = middle + 1,
                Ordering::Equal => {
                    low = middle as i32;
                    // 即使当前元素等于 target, 也要调整右侧的索引向左移动
                    right = middle - 1;
                }
                // 这里不需使用 saturating_sub() 防止右侧索引 underflow,
                // 因为 middle >= left >= 1
                Ordering::Greater => right = middle - 1,
            }
        }
        low
    }

    fn search_right(nums: &[i32], target: i32) -> i32 {
        let len = nums.len();
        // 极端情况
        if len == 0 || nums[0] > target || nums[len - 1] < target {
            return -1;
        }

        // 极端情况
        if nums[len - 1] == target {
            return len as i32 - 1;
        }

        let mut left = 0;
        let mut right = len - 2;
        let mut high: i32 = -1;
        while left <= right {
            let middle = left + (right - left) / 2;
            match nums[middle].cmp(&target) {
                Ordering::Less => left = middle + 1,
                Ordering::Equal => {
                    high = middle as i32;
                    // 即使当前元素等于 target, 也要调整左侧索引向右移动
                    left = middle + 1;
                }
                // 这里使用 saturating_sub() 防止右侧索引 underflow
                Ordering::Greater => right = middle.saturating_sub(1),
            }
        }
        high
    }

    vec![search_left(&nums, target), search_right(&nums, target)]
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
    check_solution(search_range3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, search_range1, search_range2, search_range3};

    #[test]
    fn test_search_range1() {
        check_solution(search_range1);
    }

    #[test]
    fn test_search_range2() {
        check_solution(search_range2);
    }

    #[test]
    fn test_search_range3() {
        check_solution(search_range3);
    }
}
