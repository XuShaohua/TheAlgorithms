// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! 与 0724 问题相同, 我们只写第二种解法

pub fn solution1(nums: Vec<i32>) -> i32 {
    let mut suffix_sum: i32 = nums.iter().sum();
    let mut prefix_sum: i32 = 0;
    for (index, num) in nums.iter().enumerate() {
        suffix_sum -= num;
        if prefix_sum == suffix_sum {
            return index as i32;
        }
        prefix_sum += num;
    }
    -1
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 3, -1, 8, 4];
    assert_eq!(func(nums), 3);

    let nums = vec![1, -1, 4];
    assert_eq!(func(nums), 2);

    let nums = vec![2, 5];
    assert_eq!(func(nums), -1);
}

fn main() {
    check_solution(solution1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, solution1};

    #[test]
    fn test_solution1() {
        check_solution(solution1);
    }
}
