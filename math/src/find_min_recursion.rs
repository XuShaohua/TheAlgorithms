// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Find min value in list.
///
/// Divide and Conquer algorithm impl
#[must_use]
pub const fn find_min(nums: &[i32], left: usize, right: usize) -> i32 {
    debug_assert!(!nums.is_empty());
    debug_assert!(left < nums.len());
    debug_assert!(right < nums.len());
    if left == right {
        return nums[left];
    }
    // the middle;
    let mid: usize = (left + right) >> 1;
    // find min in range[left, mid];
    let left_min = find_min(nums, left, mid);
    // find min in range[mid + 1, right];
    let right_min = find_min(nums, mid + 1, right);

    if left_min <= right_min {
        left_min
    } else {
        right_min
    }
}

#[cfg(test)]
mod tests {
    use super::find_min;

    #[test]
    fn test_find_min() {
        for nums in &[[3, 2, 1], [-3, -2, -1], [3, -3, 0], [30, 31, 29]] {
            assert_eq!(
                find_min(nums, 0, nums.len() - 1),
                *nums.iter().min().unwrap()
            );
        }
        let nums = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        assert_eq!(
            find_min(&nums, 0, nums.len() - 1),
            *nums.iter().min().unwrap()
        );
    }
}
