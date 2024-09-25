// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// Brute force
// 时间复杂度 O(n^2), 空间复杂度 O(1)
fn max_sub_array1(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            max_sum = max_sum.max(sum);
        }
    }
    max_sum
}

// DP

// Kadane

// Divide & Conquer

pub type Func = fn(Vec<i32>) -> i32;

fn check_solution(func: Func) {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(func(nums), 6);

    let nums = vec![1];
    assert_eq!(func(nums), 1);

    let nums = vec![-1];
    assert_eq!(func(nums), -1);

    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(func(nums), 23);
}

fn main() {
    check_solution(max_sub_array1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, max_sub_array1};

    #[test]
    fn test_max_sub_array1() {
        check_solution(max_sub_array1);
    }
}
