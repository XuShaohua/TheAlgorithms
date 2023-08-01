// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Find max value in list
///
/// Divide and Conquer algorithm
#[must_use]
pub const fn find_max(nums: &[i32], left: usize, right: usize) -> i32 {
    debug_assert!(!nums.is_empty());
    debug_assert!(left < nums.len());
    debug_assert!(right < nums.len());
    if left == right {
        return nums[left];
    }
    // the middle
    let mid = (left + right) >> 1;
    // find max in range[left, mid]
    let left_max = find_max(nums, left, mid);
    // find max in range[mid + 1, right]
    let right_max = find_max(nums, mid + 1, right);

    if left_max >= right_max {
        left_max
    } else {
        right_max
    }
}

#[cfg(test)]
mod tests {
    use super::find_max;

    #[test]
    fn test_find_max() {
        for nums in &[[3, 2, 1], [-3, -2, -1], [3, -3, 0], [30, 31, 29]] {
            assert_eq!(
                find_max(nums, 0, nums.len() - 1),
                *nums.iter().max().unwrap()
            );
        }
        let nums = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        assert_eq!(
            find_max(&nums, 0, nums.len() - 1),
            *nums.iter().max().unwrap()
        );
    }
}
