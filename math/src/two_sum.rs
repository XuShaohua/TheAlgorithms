// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Given an array of integers, return indices of the two numbers such that they add up to
//! a specific target.

#[must_use]
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for (index_1, val_1) in nums.iter().enumerate() {
        for (index_2, val_2) in nums.iter().enumerate() {
            if *val_1 + *val_2 == target {
                return Some((index_1, index_2));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(&[15, 2, 11, 7], 13), Some((1, 2)));
        assert_eq!(two_sum(&[2, 7, 11, 15], 17), Some((0, 3)));
        assert_eq!(two_sum(&[7, 15, 11, 2], 18), Some((0, 2)));
        assert_eq!(two_sum(&[2, 7, 11, 15], 26), Some((2, 3)));
        assert_eq!(two_sum(&[2, 7, 11, 15], 8), None);
        let nums = (0..=10).into_iter().map(|i| i * 3).collect::<Vec<_>>();
        assert_eq!(two_sum(&nums, 19), None);
    }
}
