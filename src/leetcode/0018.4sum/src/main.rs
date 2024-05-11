// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::cmp::Ordering;

// Brute force
pub fn four_sum1(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let len = nums.len();
    assert!((1..=200).contains(&len));
    if len < 4 {
        return vec![];
    }

    // 转换成 i64, 不需要处理加法溢出.
    let mut nums: Vec<i64> = nums.into_iter().map(Into::into).collect();
    // 先排序, 从小到大的顺序
    nums.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let target: i64 = target as i64;

    for i in 0..(len - 3) {
        for j in (i + 1)..(len - 2) {
            for k in (j + 1)..(len - 1) {
                for l in (k + 1)..len {
                    if nums[i] + nums[j] + nums[k] + nums[l] == target {
                        let list = vec![
                            nums[i] as i32,
                            nums[j] as i32,
                            nums[k] as i32,
                            nums[l] as i32,
                        ];
                        if !result.contains(&list) {
                            result.push(list);
                        }
                    }
                }
            }
        }
    }
    result
}

// 靠拢型双指针
pub fn four_sum2(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    assert!(!nums.is_empty());

    if nums.len() < 4 {
        return vec![];
    }

    // 转换成 i64, 更方便求和.
    let mut nums = nums;
    // 排序
    nums.sort_unstable();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let len: usize = nums.len();
    let target_i64: i64 = target as i64;

    // 遍历数组
    // 这里, 使用两层嵌套循环
    for i in 0..(len - 3) {
        // 跳过重复元素
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in (i + 1)..(len - 2) {
            // 跳过重复元素
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            // 初始化双指针.
            let mut left = j + 1;
            let mut right = len - 1;
            while left < right {
                // 不考虑整数溢出.
                let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                match sum.cmp(&target_i64) {
                    Ordering::Equal => {
                        let list = vec![nums[i], nums[j], nums[left], nums[right]];
                        if !result.contains(&list) {
                            result.push(list);
                        }
                        // 同时移动两个指针
                        left += 1;
                        right -= 1;
                    }
                    // 移动指针
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                }
            }
        }
    }

    result
}

pub type SolutionFn = fn(Vec<i32>, i32) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let out = func(nums, target);
    assert_eq!(
        out,
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );

    let nums = vec![2, 2, 2, 2, 2];
    let target = 8;
    let out = func(nums, target);
    assert_eq!(out, vec![vec![2, 2, 2, 2]]);

    let empty = Vec::<Vec<i32>>::new();

    let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
    let target = -294967296;
    let out = func(nums, target);
    assert_eq!(out, empty);

    let nums = vec![1, -2, -5, -4, -3, 3, 3, 5];
    let target = -11;
    let out = func(nums, target);
    assert_eq!(out, vec![vec![-5, -4, -3, 1]]);
}

fn main() {
    check_solution(four_sum1);
    check_solution(four_sum2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, four_sum1, four_sum2};

    #[test]
    fn test_four_sum1() {
        check_solution(four_sum1);
    }

    #[test]
    fn test_four_sum2() {
        check_solution(four_sum2);
    }
}
