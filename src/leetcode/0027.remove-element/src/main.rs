// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    while i < nums.len() {
        if nums[i] == val {
            nums.swap_remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let len = remove_element(&mut nums, val);
    assert_eq!(len, 2);
    assert_eq!(nums, vec![2, 2]);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let len = remove_element(&mut nums, val);
    assert_eq!(len, 5);
    assert_eq!(nums, vec![0, 1, 4, 0, 3]);
}
