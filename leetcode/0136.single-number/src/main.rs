// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// num ^ num = 0
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in &nums {
        ans ^= num;
    }
    ans
}

fn main() {
    let nums = vec![2, 2, 1];
    assert_eq!(single_number(nums), 1);

    let nums = vec![4, 1, 2, 1, 2];
    assert_eq!(single_number(nums), 4);

    let nums = vec![1];
    assert_eq!(single_number(nums), 1);
}
