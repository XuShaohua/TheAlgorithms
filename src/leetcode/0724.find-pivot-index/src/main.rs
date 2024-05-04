// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn pivot_index1(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut partial_sum: i32 = 0;
    for (index, num) in nums.iter().enumerate() {
        if partial_sum * 2 + num == sum {
            return index as i32;
        }
        partial_sum += num;
    }
    -1
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 7, 3, 6, 5, 6];
    assert_eq!(func(nums), 3);

    let nums = vec![1, 2, 3];
    assert_eq!(func(nums), -1);

    let nums = vec![2, 1, -1];
    assert_eq!(func(nums), 0);

    let nums = vec![1, 0];
    assert_eq!(func(nums), 0);
}

fn main() {
    check_solution(pivot_index1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, pivot_index1};

    #[test]
    fn test_solution1() {
        check_solution(pivot_index1);
    }
}
