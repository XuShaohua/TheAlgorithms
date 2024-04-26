// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// 朴树的双指针法, 手动跳过所有重复的元素
#[allow(clippy::ptr_arg)]
pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
    let mut slow = 0;

    let mut i = 0;
    while i < nums.len() {
        // 跳过重复的元素
        while i + 1 < nums.len() && nums[i] == nums[i + 1] {
            i += 1;
        }
        nums[slow] = nums[i];
        slow += 1;
        i += 1;
    }

    slow as i32
}

// 双指针 Two pointers
#[allow(clippy::ptr_arg)]
pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    // 第一个指针, 用于记录当前不重复的位置
    let mut idx = 1;
    // 第二个指针, 用于遍历数组
    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            nums[idx] = nums[i];
            idx += 1;
        }
    }
    idx as i32
}

// Vec 的去重函数, 支持已排序的
pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

pub type SolutionFn = fn(&mut Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let expected_nums = &[0, 1, 2, 3, 4];
    let k = func(&mut nums);
    assert_eq!(k, 5);
    assert_eq!(&nums[0..(k as usize)], expected_nums);

    let mut nums = vec![1, 1, 2];
    let expected_nums = &[1, 2];
    let k = func(&mut nums);
    assert_eq!(k, 2);
    assert_eq!(&nums[0..(k as usize)], expected_nums);
}

fn main() {
    check_solution(remove_duplicates1);
    check_solution(remove_duplicates2);
    check_solution(remove_duplicates3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, remove_duplicates1, remove_duplicates2, remove_duplicates3};

    #[test]
    fn test_remove_duplicates1() {
        check_solution(remove_duplicates1);
    }

    #[test]
    fn test_remove_duplicates2() {
        check_solution(remove_duplicates2);
    }

    #[test]
    fn test_remove_duplicates3() {
        check_solution(remove_duplicates3);
    }
}
