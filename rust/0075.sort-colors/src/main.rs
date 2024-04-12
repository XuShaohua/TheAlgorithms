// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = nums.len() - 1;
    while mid <= high {
        if nums[mid] == 0 {
            nums.swap(mid, low);
            low += 1;
            mid += 1;
        } else if nums[mid] == 2 {
            nums.swap(mid, high);
            if high > 0 {
                high -= 1;
            } else {
                break;
            }
        } else {
            mid += 1;
        }
    }
}

fn check_solution() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    assert_eq!(&nums, &[0, 0, 1, 1, 2, 2]);

    let mut nums = vec![2, 2];
    sort_colors(&mut nums);
    assert_eq!(&nums, &[2, 2]);

    let mut nums = vec![2];
    sort_colors(&mut nums);
    assert_eq!(&nums, &[2]);
}

fn main() {
    check_solution();
}
