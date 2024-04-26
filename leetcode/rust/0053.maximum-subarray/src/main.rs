// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [Maximum subarray](https://leetcode.com/problems/maximum-subarray/)

fn solution1(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let first_item = nums[0];
    let mut max_sum = first_item;
    let mut this_sum = 0;
    for val in nums {
        this_sum += val;
        if this_sum > max_sum {
            max_sum = this_sum;
        } else if this_sum < first_item {
            this_sum = 0;
        }
    }
    max_sum
}

fn main() {
    let nums = &[-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(solution1(nums), 6);

    let nums = &[1];
    assert_eq!(solution1(nums), 1);

    let nums = &[-1];
    assert_eq!(solution1(nums), -1);

    let nums = &[5, 4, -1, 7, 8];
    assert_eq!(solution1(nums), 23);
}

#[cfg(test)]
mod tests {
    use super::solution1;

    #[test]
    fn test_solution1() {
        let nums = &[-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(solution1(nums), 6);

        let nums = &[1];
        assert_eq!(solution1(nums), 1);

        let nums = &[-1];
        assert_eq!(solution1(nums), -1);

        let nums = &[5, 4, -1, 7, 8];
        assert_eq!(solution1(nums), 23);
    }
}
