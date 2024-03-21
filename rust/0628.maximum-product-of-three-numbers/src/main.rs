// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    assert!(len >= 3);

    nums.sort();
    let mut max_vals = [
        // -3, -2, -1
        nums[len - 3] * nums[len - 2] * nums[len - 1],
        // 0, -2, -1
        nums[0] * nums[len - 2] * nums[len - 1],
        // 0, 1, -1
        nums[0] * nums[1] * nums[len - 1],
    ];
    max_vals.sort();
    max_vals[max_vals.len() - 1]
}

fn check_solution() {
    let nums = vec![-100, -98, -1, 2, 3, 4];
    assert_eq!(maximum_product(nums), 39200);
}

fn main() {
    check_solution();
}
