// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::ops::Add;

pub fn prefix_sum<T>(arr: &[T]) -> Vec<T>
where
    T: Clone + Add<T, Output=T>,
{
    if arr.is_empty() {
        return vec![];
    }
    let mut list = Vec::with_capacity(arr.len());
    list.push(arr[0].clone());
    for i in 1..arr.len() {
        list.push(arr[i].clone() + list[i - 1].clone());
    }
    debug_assert!(list.len() == arr.len());
    list
}

#[cfg(test)]
mod tests {
    use super::prefix_sum;

    #[test]
    fn test_prefix_sum_basic() {
        let nums = [1, 2, 3, 4, 5];
        let prefix_sum_array = prefix_sum(&nums);
        assert_eq!(prefix_sum_array, [1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_prefix_sum_single() {
        let nums = [10];
        let prefix_sum_array = prefix_sum(&nums);
        assert_eq!(prefix_sum_array, [10]);
    }
}
