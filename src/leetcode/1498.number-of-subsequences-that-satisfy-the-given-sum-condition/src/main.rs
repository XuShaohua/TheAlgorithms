// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn num_subseq1(nums: Vec<i32>, target: i32) -> i32 {
    assert!(!nums.is_empty());
    assert!(target >= 1);

    todo!()
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 5, 6, 7];
    let target = 9;
    assert_eq!(func(nums, target), 4);

    let nums = vec![3, 3, 6, 8];
    let target = 10;
    assert_eq!(func(nums, target), 6);

    let nums = vec![2, 3, 3, 4, 6, 7];
    let target = 12;
    assert_eq!(func(nums, target), 61);
}

fn main() {
    check_solution(num_subseq1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, num_subseq1};

    #[test]
    fn test_num_subseq1() {
        check_solution(num_subseq1);
    }
}
