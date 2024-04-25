// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::collections::HashMap;

// Brute Force
pub fn contains_nearby_duplicate1(nums: Vec<i32>, k: i32) -> bool {
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

// 使用 HashMap
pub fn contains_nearby_duplicate2(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;

    // map 用于存储数值及其在数组中的位置
    let mut map = HashMap::<i32, usize>::with_capacity(nums.len());

    // 遍历数组中所有元素
    for (index, &num) in nums.iter().enumerate() {
        // 在 map 中尝试查找这个元素, 并判断与当前遍历的元素是否重复.
        if let Some(&old_index) = map.get(&num) {
            if (index - old_index) <= k {
                return true;
            }
        }
        map.insert(num, index);
    }
    false
}

pub type SolutionFn = fn(Vec<i32>, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    assert!(func(nums, k));

    let nums = vec![1, 0, 1, 1];
    let k = 1;
    assert!(func(nums, k));

    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;
    assert!(!func(nums, k));
}

fn main() {
    check_solution(contains_nearby_duplicate1);
    check_solution(contains_nearby_duplicate2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, contains_nearby_duplicate1};

    #[test]
    fn test_contains_nearby_duplicate1() {
        check_solution(contains_nearby_duplicate1);
    }
}
