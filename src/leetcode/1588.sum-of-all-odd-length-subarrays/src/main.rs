// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Prefix Sum
pub fn sum_odd_length_subarrays1(arr: Vec<i32>) -> i32 {
    debug_assert!(!arr.is_empty());
    todo!()
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let arr = vec![1, 4, 2, 5, 3];
    assert_eq!(func(arr), 58);

    let arr = vec![1, 2];
    assert_eq!(func(arr), 3);

    let arr = vec![10, 11, 12];
    assert_eq!(func(arr), 66);
}

fn main() {
    check_solution(sum_odd_length_subarrays1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, sum_odd_length_subarrays1};

    #[test]
    fn test_sum_odd_length_subarrays1() {
        check_solution(sum_odd_length_subarrays1);
    }
}
