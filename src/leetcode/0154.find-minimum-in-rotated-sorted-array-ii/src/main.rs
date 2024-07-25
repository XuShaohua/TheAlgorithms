// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute force
pub fn find_min1(nums: Vec<i32>) -> i32 {
    nums.iter().min().copied().unwrap_or_default()
}

// Binary Search
#[allow(clippy::comparison_chain)]
pub fn find_min2(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    let mut left = 0;
    let mut right = nums.len() - 1;
    while left + 1 < right {
        let middle = left + (right - left) / 2;
        //println!("left: {left}, middle: {middle} right: {right}, nums: {nums:?}");
        if nums[middle] < nums[right] {
            // (1, 2, 3)
            // 最小值位于左侧
            right = middle;
        } else if nums[middle] > nums[right] {
            // (2, 3, 1)
            // 最小值位于右侧
            left = middle + 1;
        } else {
            // 不容易确定, 一次移动一个位置, 考虑重复的元素.
            if right > left && nums[right - 1] <= nums[right] {
                right -= 1;
            }
            if left < right && nums[left + 1] <= nums[left] {
                left += 1;
            }
        }
    }

    nums[left].min(nums[right])
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 3, 5];
    assert_eq!(func(nums), 1);

    let nums = vec![2, 2, 2, 0, 1];
    assert_eq!(func(nums), 0);

    let nums = vec![3, 1, 1];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 1, 3];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 1, 2];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 1, 3, 3];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 3, 1, 3];
    assert_eq!(func(nums), 1);

    let nums = vec![1, 3, 3];
    assert_eq!(func(nums), 1);

    let nums = vec![10, 1, 10, 10, 10];
    assert_eq!(func(nums), 1);

    let nums = vec![1, 2, 1];
    assert_eq!(func(nums), 1);
}

fn main() {
    check_solution(find_min1);
    check_solution(find_min2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_min1, find_min2};

    #[test]
    fn test_find_min1() {
        check_solution(find_min1);
    }

    #[test]
    fn test_find_min2() {
        check_solution(find_min2);
    }
}
