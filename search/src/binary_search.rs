// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#[allow(clippy::comparison_chain)]
#[must_use]
pub fn binary_search<T: PartialOrd>(nums: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] < *target {
            low = mid + 1;
        } else if nums[mid] > *target {
            high = mid - 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_binary_search() {
        const NUMS: &[i32] = &[-7, -3, -2, -2, -1, 2, 4, 5, 6];
        assert_eq!(binary_search(NUMS, &2), Some(5));
        assert_eq!(binary_search(NUMS, &1), None);
    }
}
