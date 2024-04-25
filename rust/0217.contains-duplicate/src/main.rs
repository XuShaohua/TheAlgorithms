// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::collections::HashSet;

// 两层遍历数组
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

// 先排序, 再遍历
pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
    assert!(!nums.is_empty());
    let mut nums = nums;
    // 先排序
    nums.sort();
    let len = nums.len();
    for i in 0..(len - 1) {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}

// 使用 Hash Set 存放访问过的数组中的元素.
pub fn contains_duplicate3(nums: Vec<i32>) -> bool {
    if nums.len() < 2 {
        return false;
    }
    let mut cache = HashSet::with_capacity(nums.len());
    for &num in nums.iter() {
        // 如果 insert() 返回 false, 就说明 cache 之前已经包含 num 了; 说明 num 是重复的元素.
        if !cache.insert(num) {
            return true;
        }
    }
    false
}

// 使用 Hash Set 存放数组中的所有元素, 最后只比较两者中元素的个数是否相同.
pub fn contains_duplicate4(nums: Vec<i32>) -> bool {
    let set = nums.iter().collect::<HashSet<_>>();
    set.len() != nums.len()
}

pub type SolutionFn = fn(Vec<i32>) -> bool;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 1];
    assert!(func(nums));

    let nums = vec![1, 2, 3, 4];
    assert!(!func(nums));

    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert!(func(nums));
}

fn main() {
    check_solution(contains_duplicate);
    check_solution(contains_duplicate2);
    check_solution(contains_duplicate3);
    check_solution(contains_duplicate4);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, contains_duplicate, contains_duplicate2, contains_duplicate3,
        contains_duplicate4,
    };

    #[test]
    fn test_contains_duplicate() {
        check_solution(contains_duplicate);
    }

    #[test]
    fn test_contains_duplicate2() {
        check_solution(contains_duplicate2);
    }

    #[test]
    fn test_contains_duplicate3() {
        check_solution(contains_duplicate3);
    }

    #[test]
    fn test_contains_duplicate4() {
        check_solution(contains_duplicate4);
    }
}
