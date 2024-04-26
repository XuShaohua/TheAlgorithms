// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Sort array and filter based on rules.
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

/// Find 3 maximum and 2 minimum directly, no sorting.
pub fn maximum_product2(nums: Vec<i32>) -> i32 {
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;
    let mut max3 = i32::MIN;
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;
    for num in nums {
        if num > max1 {
            max3 = max2;
            max2 = max1;
            max1 = num;
        } else if num > max2 {
            max3 = max2;
            max2 = num;
        } else if num > max3 {
            max3 = num;
        }

        if num < min1 {
            min2 = min1;
            min1 = num;
        } else if num < min2 {
            min2 = num;
        }
    }

    (min1 * min2 * max1).max(max1 * max2 * max3)
}

fn check_solution() {
    let nums = vec![-100, -98, -1, 2, 3, 4];
    assert_eq!(maximum_product2(nums), 39200);
}

fn main() {
    check_solution();
}
