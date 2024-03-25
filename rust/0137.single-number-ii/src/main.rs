// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// bit vector
pub fn single_number(nums: Vec<i32>) -> i32 {
    const DIGIT_LEN: usize = 32;

    let mut ans = 0;
    for i in 0..DIGIT_LEN {
        let sum = nums.iter().map(|num| num >> i & 1).sum::<i32>();
        ans |= (sum % 3) << i;
    }
    ans
}

fn check_solution() {
    let nums = vec![2, 2, 3, 2];
    assert_eq!(single_number(nums), 3);

    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(single_number(nums), 99);
}

fn main() {
    check_solution();
}
