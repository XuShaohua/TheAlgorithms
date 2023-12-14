// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [two-sum](https://leetcode.com/problems/two-sum)

use std::collections::HashMap;

fn solution1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for (i, m) in nums.iter().enumerate() {
        for (j, n) in nums.iter().enumerate() {
            if i != j && m + n == target {
                result.push(i as i32);
                result.push(j as i32);
                return result;
            }
        }
    }
    result
}

fn solution2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if i != j && nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    Vec::new()
}

fn solution3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut visited = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        if let Some(&j) = visited.get(&(target - nums[i])) {
            return vec![j as i32, i as i32];
        } else {
            visited.insert(nums[i], i);
        }
    }

    Vec::new()
}

fn check_solution1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(solution1(nums, target), vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(solution1(nums, target), vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(solution1(nums, target), vec![0, 1]);
}

fn check_solution2() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(solution2(nums, target), vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(solution2(nums, target), vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(solution2(nums, target), vec![0, 1]);
}

fn check_solution3() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(solution3(nums, target), vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(solution3(nums, target), vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(solution3(nums, target), vec![0, 1]);
}

fn main() {
    check_solution1();
    check_solution2();
    check_solution3();
}

#[cfg(test)]
mod tests {
    use super::{check_solution1, check_solution2, check_solution3};

    #[test]
    fn test_solution1() {
        check_solution1();
    }

    #[test]
    fn test_solution2() {
        check_solution2();
    }

    #[test]
    fn test_solution3() {
        check_solution3();
    }
}
