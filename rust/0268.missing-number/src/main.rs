// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut current = 0;
    for (i, num) in nums.iter().enumerate() {
        let i = i as i32;
        current = current ^ num ^ i;
    }
    current = current ^ (nums.len() as i32);
    current
}

fn check_solution() {
    let nums = vec![3, 0, 1];
    assert_eq!(missing_number(nums), 2);

    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(missing_number(nums), 8);
}

fn main() {
    check_solution();
}
