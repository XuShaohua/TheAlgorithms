// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Prefix Sum
pub fn left_right_difference1(nums: Vec<i32>) -> Vec<i32> {
    debug_assert!(!nums.is_empty());

    let len = nums.len();
    let mut left_sum = vec![0; len];
    left_sum[0] = 0;
    // 从左向右遍历
    for i in 0..(len - 1) {
        left_sum[i + 1] = left_sum[i] + nums[i];
    }
    //println!("left sum: {left_sum:?}");

    let mut right_sum = vec![0; len];
    right_sum[0] = 0;
    // 从右向左遍历
    for i in (1..=(len - 1)).rev() {
        right_sum[i - 1] = right_sum[i] + nums[i];
    }
    //println!("right sum: {right_sum:?}");

    left_sum
        .into_iter()
        .zip(right_sum)
        .map(|(left, right)| (left - right).abs())
        .collect()
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![10, 4, 8, 3];
    assert_eq!(func(nums), [15, 1, 11, 22]);

    let nums = vec![1];
    assert_eq!(func(nums), [0]);
}

fn main() {
    check_solution(left_right_difference1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, left_right_difference1};

    #[test]
    fn test_left_right_difference1() {
        check_solution(left_right_difference1);
    }
}
