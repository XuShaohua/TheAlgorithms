// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let len = nums.len();
    assert!(len >= 4);
    nums.sort();

    let mut result = Vec::new();
    for i in 0..(len - 3) {
        for j in (i + 1)..(len - 2) {
            for k in (j + 1)..(len - 1) {
                for l in (k + 1)..len {
                    if nums[i] + nums[j] + nums[k] + nums[l] == target {
                        let list = vec![nums[i], nums[j], nums[k], nums[l]];
                        if !result.contains(&list) {
                            result.push(list);
                        }
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let out = four_sum(nums, target);
    assert_eq!(
        out,
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );

    let nums = vec![2, 2, 2, 2, 2];
    let target = 8;
    let out = four_sum(nums, target);
    assert_eq!(out, vec![vec![2, 2, 2, 2]]);
}
