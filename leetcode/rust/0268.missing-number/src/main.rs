// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub type SolutionFunc = fn(Vec<i32>) -> i32;

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut current = 0;
    for (i, num) in nums.iter().enumerate() {
        let i = i as i32;
        current = current ^ num ^ i;
    }
    current ^= nums.len() as i32;
    current
}

pub fn missing_number2(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    (n * (n + 1)) / 2 - nums.iter().sum::<i32>()
}

fn check_solution(func: SolutionFunc) {
    let nums = vec![3, 0, 1];
    assert_eq!(func(nums), 2);

    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(func(nums), 8);
}

fn main() {
    check_solution(missing_number);
    check_solution(missing_number2);
}
