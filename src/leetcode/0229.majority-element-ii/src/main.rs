// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
// 字典计数
pub fn majority_element1(nums: Vec<i32>) -> Vec<i32> {
    debug_assert!(!nums.is_empty());

    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    let mut out = Vec::new();

    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let len: usize = nums.len();
    for (num, count) in map {
        if count * 3 > len {
            out.push(num);
        }
    }

    out
}

// TODO(Shaohua): 摩尔投票法

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 2, 3];
    assert_eq!(func(nums), vec![3]);

    let nums = vec![1];
    assert_eq!(func(nums), vec![1]);

    let nums = vec![1, 2];
    assert_eq!(func(nums), vec![1, 2]);
}

fn main() {
    check_solution(majority_element1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, majority_element1};

    #[test]
    fn test_majority_element1() {
        check_solution(majority_element1);
    }
}
