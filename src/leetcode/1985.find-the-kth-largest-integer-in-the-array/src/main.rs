// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;

// slice::select_nth_unstable()
pub fn kth_largest_number1(nums: Vec<String>, k: i32) -> String {
    let mut nums = nums;
    let k_small: usize = nums.len() - k as usize;
    nums.select_nth_unstable_by(k_small, |a, b| match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(b),
        order => order,
    })
    .1
    .to_owned()
}

pub type SolutionFn = fn(Vec<String>, i32) -> String;

fn check_solution(func: SolutionFn) {
    let nums = vec![
        "3".to_owned(),
        "6".to_owned(),
        "7".to_owned(),
        "10".to_owned(),
    ];
    assert_eq!(func(nums, 4), "3".to_owned());

    let nums = vec![
        "2".to_owned(),
        "21".to_owned(),
        "12".to_owned(),
        "1".to_owned(),
    ];
    assert_eq!(func(nums, 3), "2".to_owned());

    let nums = vec!["0".to_owned(), "0".to_owned()];
    assert_eq!(func(nums, 2), "0".to_owned());
}

fn main() {
    check_solution(kth_largest_number1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, kth_largest_number1};

    #[test]
    fn test_kth_largest_number1() {
        check_solution(kth_largest_number1);
    }
}
