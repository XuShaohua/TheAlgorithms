// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut ans = 0;
    let mut seq_len = 1;
    for i in 0..(nums.len() - 1) {
        if nums[i] + 1 == nums[i + 1] {
            seq_len += 1;
        } else {
            ans = ans.max(seq_len);
            seq_len = 1;
        }
    }
    ans = ans.max(seq_len);
    ans
}

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    assert_eq!(longest_consecutive(nums), 4);

    let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    assert_eq!(longest_consecutive(nums), 9);
}
