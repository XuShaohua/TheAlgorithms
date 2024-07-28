// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Prefix Sum
pub fn return_to_boundary_count1(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());

    let mut positions = vec![0; nums.len()];
    positions[0] = nums[0];
    for i in 1..nums.len() {
        positions[i] = positions[i - 1] + nums[i];
    }
    positions.into_iter().filter(|&x| x == 0).count() as i32
}

// Prefix Sum
// 不保存中间值到数组
pub fn return_to_boundary_count2(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());

    let mut zeros_count = 0;
    let mut last_position = 0;
    for num in nums {
        last_position += num;
        if last_position == 0 {
            zeros_count += 1;
        }
    }
    zeros_count
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 3, -5];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 2, -3, -4];
    assert_eq!(func(nums), 0);
}

fn main() {
    check_solution(return_to_boundary_count1);
    check_solution(return_to_boundary_count2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, return_to_boundary_count1, return_to_boundary_count2};

    #[test]
    fn test_return_to_boundary_count1() {
        check_solution(return_to_boundary_count1);
    }

    #[test]
    fn test_return_to_boundary_count2() {
        check_solution(return_to_boundary_count2);
    }
}
