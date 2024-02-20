// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    let len = nums.len();
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            for k in j..len {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut ok_nums = vec![nums[i], nums[j], nums[k]];
                    ok_nums.sort();
                    if !result.contains(&ok_nums) {
                        result.push(ok_nums);
                    }
                }
            }
        }
    }

    result.sort();
    result
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let out = three_sum(nums);
    assert_eq!(out, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    let nums = vec![0, 1, 1];
    let out = three_sum(nums);
    assert!(out.is_empty());

    let nums = vec![0, 0, 0];
    let out = three_sum(nums);
    assert_eq!(out, vec![vec![0, 0, 0]]);
}
