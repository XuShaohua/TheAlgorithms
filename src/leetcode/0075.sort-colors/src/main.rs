// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

// 靠拢型双指针
// Dutch National Flag, DNF
// three-way partition
pub fn sort_colors1(nums: &mut Vec<i32>) {
    assert!(!nums.is_empty());

    // 双指针的变形, 三指针
    // left 用于指向数组中为0的元素的右侧
    let mut left = 0;
    // mid 用于遍历数组
    let mut mid = 0;
    // right 用于指向数组中为2的元素的左侧
    let mut right = nums.len() - 1;

    // 遍历数组
    while mid <= right {
        if nums[mid] == 0 {
            nums.swap(mid, left);
            mid += 1;
            // 左边的指针往右移一下
            left += 1;
        } else if nums[mid] == 2 {
            nums.swap(mid, right);
            // 右边的指针往左移一下
            if right > 0 {
                right -= 1;
            } else {
                break;
            }
        } else {
            mid += 1;
        }
    }
}

// 调用数组的排序方法, 这个不符合要求.
pub fn sort_colors2(nums: &mut Vec<i32>) {
    nums.sort();
}

// 选择排序 Selection Sort
pub fn sort_colors3(nums: &mut Vec<i32>) {
    for i in 0..(nums.len() - 1) {
        for j in i..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 插入排序 Insertion Sort
pub fn sort_colors4(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j - 1] > nums[j] {
            nums.swap(j - 1, j);
            j -= 1;
        }
    }
}

// 冒泡排序 Bubble Sort
pub fn sort_colors5(nums: &mut Vec<i32>) {
    for i in 0..(nums.len() - 1) {
        let mut swapped = false;
        for j in 0..(nums.len() - 1 - i) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub type SolutionFn = fn(&mut Vec<i32>);

fn check_solution(func: SolutionFn) {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    func(&mut nums);
    assert_eq!(&nums, &[0, 0, 1, 1, 2, 2]);

    let mut nums = vec![2, 2];
    func(&mut nums);
    assert_eq!(&nums, &[2, 2]);

    let mut nums = vec![2];
    func(&mut nums);
    assert_eq!(&nums, &[2]);
}

fn main() {
    check_solution(sort_colors1);
    check_solution(sort_colors2);
    check_solution(sort_colors3);
    check_solution(sort_colors4);
    check_solution(sort_colors5);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, sort_colors1, sort_colors2, sort_colors3, sort_colors4, sort_colors5,
    };

    #[test]
    fn test_sort_colors1() {
        check_solution(sort_colors1);
    }

    #[test]
    fn test_sort_colors2() {
        check_solution(sort_colors2);
    }

    #[test]
    fn test_sort_colors3() {
        check_solution(sort_colors3);
    }

    #[test]
    fn test_sort_colors4() {
        check_solution(sort_colors4);
    }

    #[test]
    fn test_sort_colors5() {
        check_solution(sort_colors5);
    }
}
