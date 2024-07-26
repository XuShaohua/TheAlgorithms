// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute Force
// O(n)
pub fn find_peak_element1(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());
    if nums.len() == 1 {
        return 0;
    }
    // 先处理边界情况.
    if nums[0] > nums[1] {
        return 0;
    }
    let last = nums.len() - 1;
    if nums[last - 1] < nums[last] {
        return last as i32;
    }

    for i in 1..last {
        if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] {
            return i as i32;
        }
    }

    -1
}

// Binary Search
pub fn find_peak_element2(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());

    if nums.len() == 1 {
        return 0;
    }

    // 先处理边界情况.
    // 这里先检查第一个和最后一个元素.
    if nums[0] > nums[1] {
        return 0;
    }
    let last = nums.len() - 1;
    if nums[last - 1] < nums[last] {
        return last as i32;
    }

    // 使用二分法找一个峰值, 检查数组中剩下的元素.
    let mut left = 1;
    let mut right = last - 1;
    while left <= right {
        // 峰值出现的位置与 nums[middle] 的关系有三种情况.
        let middle = left + (right - left) / 2;
        if nums[middle] > nums[middle + 1] && middle > 0 && nums[middle] > nums[middle - 1] {
            // 1. middle 处就是峰值
            return middle as i32;
        }
        if nums[middle] < nums[middle + 1] {
            // 2. 峰值在 middle 的右侧
            left = middle + 1;
        } else if middle > 0 && nums[middle] < nums[middle - 1] {
            // 3. 峰值在 middle 的左侧
            right = middle - 1;
        }
    }

    -1
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(func(nums), 2);

    let nums = vec![1, 2, 1, 3, 5, 6, 4];
    let ret = func(nums);
    assert!(ret == 5 || ret == 1);

    let nums = vec![1];
    assert_eq!(func(nums), 0);

    let nums = vec![3, 4, 3, 2, 1];
    assert_eq!(func(nums), 1);

    let nums = vec![1, 2, 1, 2, 1];
    let ret = func(nums);
    assert!(ret == 1 || ret == 3);
}

fn main() {
    check_solution(find_peak_element1);
    check_solution(find_peak_element2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_peak_element1, find_peak_element2};

    #[test]
    fn test_find_peak_element1() {
        check_solution(find_peak_element1);
    }

    #[test]
    fn test_find_peak_element2() {
        check_solution(find_peak_element2);
    }
}
