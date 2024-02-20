// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    assert!(nums.len() >= 3);
    let mut closest = nums[0] + nums[1] + nums[2];
    let len = nums.len();
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            for k in (j + 1)..len {
                let sum = nums[i] + nums[j] + nums[k];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
            }
        }
    }
    closest
}

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    assert_eq!(three_sum_closest(nums, target), 2);

    let nums = vec![0, 0, 0];
    let target = 1;
    assert_eq!(three_sum_closest(nums, target), 0);
}
