// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Sorting
// 如果该数出现次数过半, 那么当数组排序后, 位于数组中间的那个数, 一定就是它.
pub fn majority_element1(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len() / 2]
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 2, 3];
    assert_eq!(func(nums), 3);

    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(func(nums), 2);
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
