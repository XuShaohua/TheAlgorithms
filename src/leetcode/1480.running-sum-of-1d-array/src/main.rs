// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Prefix sum
pub fn running_sum1(nums: Vec<i32>) -> Vec<i32> {
    debug_assert!(!nums.is_empty());
    let mut prefix = vec![0; nums.len()];
    prefix[0] = nums[0];
    for i in 1..nums.len() {
        prefix[i] = prefix[i - 1] + nums[i];
    }
    prefix
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(func(nums), [1, 3, 6, 10]);

    let nums = vec![1, 1, 1, 1, 1];
    assert_eq!(func(nums), [1, 2, 3, 4, 5]);

    let nums = vec![3, 1, 2, 10, 1];
    assert_eq!(func(nums), [3, 4, 6, 16, 17]);
}

fn main() {
    check_solution(running_sum1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, running_sum1};

    #[test]
    fn test_running_sum1() {
        check_solution(running_sum1);
    }
}
