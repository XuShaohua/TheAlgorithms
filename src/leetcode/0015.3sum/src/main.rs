// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

// 暴力法
// 这个方法时间复杂度为 O(n^3), 空间复杂度为 O(n)
// 计算结果超时
pub fn three_sum1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let len = nums.len();
    if len < 3 {
        return vec![];
    }

    let mut set = HashSet::new();

    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            for k in (j + 1)..len {
                if nums[i] + nums[j] + nums[k] == 0 {
                    set.insert(vec![nums[i], nums[j], nums[k]]);
                }
            }
        }
    }
    set.into_iter().collect()
}

// 靠拢型双指针
// 这个方法时间复杂度为 O(n^2), 空间复杂度为 O(1)
pub fn three_sum2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    // 先排序
    nums.sort();

    let mut result = Vec::new();
    let len = nums.len();
    if len < 3 {
        return result;
    }

    // 遍历数组
    for i in 0..(len - 2) {
        let first = nums[i];

        // 忽略掉第一个元素大于0的值
        if first > 0 {
            break;
        }
        // 跳过重复的元素
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // 初始化双指针, 分别指向子数组的左右两端.
        let mut left = i + 1;
        let mut right = len - 1;

        // 循环中止的条件是两指针重叠.
        while left < right {
            let sum = first + nums[left] + nums[right];
            match sum.cmp(&0) {
                // 移动其中的一个指针
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    // 其和等于0, 找到目标元素.
                    result.push(vec![first, nums[left], nums[right]]);

                    let last_left_val = nums[left];
                    // 从左右两端分别跳过重复的元素.
                    while left < right && nums[left] == last_left_val {
                        left += 1;
                    }
                    let last_right_val = nums[right];
                    while left < right && nums[right] == last_right_val {
                        right -= 1;
                    }
                }
            }
        }
    }

    result
}

// 靠拢型双指针, 并使用 HashSet 去掉重复的结果
// 这个方法时间复杂度为 O(n^2), 空间复杂度为 O(n)
pub fn three_sum3(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    // 先排序
    nums.sort();

    let mut set = HashSet::new();
    let len = nums.len();
    if len < 3 {
        return vec![];
    }

    // 遍历数组
    for i in 0..(len - 2) {
        let first = nums[i];

        // 忽略掉第一个元素大于0的值
        if first > 0 {
            break;
        }
        // 跳过重复的元素
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // 初始化双指针, 分别指向子数组的左右两端.
        let mut left = i + 1;
        let mut right = len - 1;

        // 循环中止的条件是两指针重叠.
        while left < right {
            let sum = first + nums[left] + nums[right];
            match sum.cmp(&0) {
                // 移动其中的一个指针
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    // 其和等于0, 找到目标元素.
                    set.insert(vec![first, nums[left], nums[right]]);

                    // 同时移动左右两个指针, 因为重复元素在上面已经被移除.
                    left += 1;
                    right -= 1;
                }
            }
        }
    }

    set.into_iter().collect()
}

// 使用3个字典分别存储大于0, 等于0, 以及小于0的元素
pub fn three_sum4(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut zeros = 0;
    let mut negatives = HashMap::new();
    let mut positives = HashMap::new();
    for &num in &nums {
        match num.cmp(&0) {
            Ordering::Less => {
                negatives
                    .entry(num)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                ()
            }
            Ordering::Equal => zeros += 1,
            Ordering::Greater => {
                positives
                    .entry(num)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                ()
            }
        }
    }

    let mut set = HashSet::new();

    // 首先如果0存在, 就用它作为正负数的分隔.
    if zeros > 0 {
        for &num in negatives.keys() {
            if positives.contains_key(&(-num)) {
                set.insert(vec![num, 0, -num]);
            }
        }

        if zeros > 2 {
            set.insert(vec![0, 0, 0]);
        }
    }

    // 现在考虑非0组合, 可能的情竞是:
    // - (负数, 负数, 正数)
    // - (负数, 正数, 正数)

    for &negative in negatives.keys() {
        for &positive in positives.keys() {
            let expected_num = -(positive + negative);
            if expected_num < 0 {
                if let Some(&count) = negatives.get(&expected_num) {
                    if ((count > 1) && (negative == expected_num)) || (negative != expected_num) {
                        if negative < expected_num {
                            set.insert(vec![negative, expected_num, positive]);
                        } else {
                            set.insert(vec![expected_num, negative, positive]);
                        }
                    }
                }
            } else if expected_num > 0 {
                if let Some(&count) = positives.get(&expected_num) {
                    if ((count > 1) && (positive == expected_num)) || (positive != expected_num) {
                        if positive < expected_num {
                            set.insert(vec![negative, positive, expected_num]);
                        } else {
                            set.insert(vec![negative, expected_num, positive]);
                        }
                    }
                }
            }
        }
    }

    set.into_iter().collect()
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<Vec<i32>>;

#[must_use]
#[inline]
fn nearly_eq(mut vec1: Vec<Vec<i32>>, mut vec2: Vec<Vec<i32>>) -> bool {
    vec1.sort();
    vec2.sort();
    println!("vec1: {vec1:?}, vec2: {vec2:?}");
    vec1 == vec2
}

fn check_solution(func: SolutionFn) {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let out = func(nums);
    assert!(nearly_eq(out, vec![vec![-1, -1, 2], vec![-1, 0, 1]]));

    let nums = vec![0, 1, 1];
    let out = func(nums);
    assert!(out.is_empty());

    let nums = vec![0, 0, 0];
    let out = func(nums);
    assert!(nearly_eq(out, vec![vec![0, 0, 0]]));
}

fn main() {
    check_solution(three_sum1);
    check_solution(three_sum2);
    check_solution(three_sum3);
    check_solution(three_sum4);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, three_sum1, three_sum2, three_sum3, three_sum4};

    #[test]
    fn test_three_sum1() {
        check_solution(three_sum1);
    }

    #[test]
    fn test_three_sum2() {
        check_solution(three_sum2);
    }

    #[test]
    fn test_three_sum3() {
        check_solution(three_sum3);
    }

    #[test]
    fn test_three_sum4() {
        check_solution(three_sum4);
    }
}
