// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Binary Serach
pub fn find_min1(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    // 极限情况.
    let last = nums.len() - 1;
    if nums[0] < nums[last] {
        return nums[0];
    }

    let mut left = 0;
    let mut right = last;

    while left < right {
        let middle = left + (right - left) / 2;
        // (1, 2, 3)
        if nums[left] < nums[middle] && nums[middle] < nums[right] {
            // 最小值
            return nums[left];
        }

        if nums[middle] > nums[right] {
            // (2, 3, 1)
            // 右侧部分较小
            left = middle + 1;
        } else {
            // (3, 1, 2)
            // 左侧部分较小
            right = middle;
        }
    }
    nums[left]
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 4, 5, 1, 2];
    assert_eq!(func(nums), 1);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(func(nums), 0);

    let nums = vec![11, 13, 15, 17];
    assert_eq!(func(nums), 11);
}

fn main() {
    check_solution(find_min1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_min1};

    #[test]
    fn test_find_min1() {
        check_solution(find_min1);
    }
}
