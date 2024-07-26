// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute Force
pub fn peak_index_in_mountain_array1(arr: Vec<i32>) -> i32 {
    debug_assert!(arr.len() >= 3);

    for i in 1..(arr.len() - 1) {
        if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] {
            return i as i32;
        }
    }
    -1
}

// Binary Search
pub fn peak_index_in_mountain_array2(arr: Vec<i32>) -> i32 {
    debug_assert!(arr.len() >= 3);

    // 直接忽略第一个及最后一个元素.
    let mut left = 1;
    let mut right = arr.len() - 2;
    while left <= right {
        let middle = left + (right - left) / 2;
        // 峰值出现的位置有三种可能.
        if arr[middle] > arr[middle + 1] && middle > 0 && arr[middle] > arr[middle - 1] {
            // 1. middle 处就是峰值
            return middle as i32;
        }
        if arr[middle] < arr[middle + 1] {
            // 2. 峰值位于 middle 的右侧
            left = middle + 1;
        } else {
            // 3. 峰值位于 middle 的左侧
            right = middle - 1;
        }
    }
    -1
}

// Binary Search
// 对上面的细节作了修改
pub fn peak_index_in_mountain_array3(arr: Vec<i32>) -> i32 {
    debug_assert!(arr.len() >= 3);
    let mut left = 0;
    let mut right = arr.len() - 1;

    // 简化二分查找的条件
    while left < right {
        let middle = left + (right - left) / 2;
        if arr[middle] < arr[middle + 1] {
            // 1. 峰值位于 middle 右侧
            left = middle + 1;
        } else {
            // 2. 峰值位于 middle 处或在其左侧
            right = middle;
        }
    }
    left as i32
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let arr = vec![0, 1, 0];
    assert_eq!(func(arr), 1);

    let arr = vec![0, 2, 1, 0];
    assert_eq!(func(arr), 1);

    let arr = vec![0, 10, 5, 2];
    assert_eq!(func(arr), 1);
}

fn main() {
    check_solution(peak_index_in_mountain_array1);
    check_solution(peak_index_in_mountain_array2);
    check_solution(peak_index_in_mountain_array3);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, peak_index_in_mountain_array1, peak_index_in_mountain_array2,
        peak_index_in_mountain_array3,
    };

    #[test]
    fn test_peak_index_in_mountain_array1() {
        check_solution(peak_index_in_mountain_array1);
    }

    #[test]
    fn test_peak_index_in_mountain_array2() {
        check_solution(peak_index_in_mountain_array2);
    }

    #[test]
    fn test_peak_index_in_mountain_array3() {
        check_solution(peak_index_in_mountain_array3);
    }
}
