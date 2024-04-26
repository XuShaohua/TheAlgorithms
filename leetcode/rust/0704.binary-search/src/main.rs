// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [binary search](https://leetcode.com/problems/binary-search)

use std::cmp::Ordering;

pub fn solution1(nums: &[i32], target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;
    while low <= high {
        let middle = low + (high - low) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => low = middle + 1,
            Ordering::Equal => return middle as i32,
            Ordering::Greater => {
                if middle < 1 {
                    return -1;
                } else {
                    high = middle - 1;
                }
            }
        }
    }
    -1
}

fn main() {
    let nums = &[-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(solution1(nums, target), 4);
    let nums = &[-1, 0, 3, 5, 9, 12];
    let target = 2;
    assert_eq!(solution1(nums, target), -1);
    let nums = &[];
    let target = 2;
    assert_eq!(solution1(nums, target), -1);

    let nums = &[5];
    let target = -5;
    assert_eq!(solution1(nums, target), -1);
}

#[cfg(test)]
mod tests {
    use super::solution1;

    #[test]
    fn test_solution1() {
        let nums = &[-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(solution1(nums, target), 4);

        let nums = &[-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(solution1(nums, target), -1);

        let nums = &[];
        let target = 2;
        assert_eq!(solution1(nums, target), -1);
    }
}
