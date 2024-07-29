// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;

// Hash Map
pub fn number_of_points1(nums: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut set = HashSet::new();
    for num in nums {
        debug_assert!(num.len() == 2);
        for i in num[0]..=num[1] {
            set.insert(i);
        }
    }
    set.len() as i32
}

// Bit Set
pub fn number_of_points2(nums: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut bitset = vec![false; 101];
    for num in nums {
        for i in num[0]..=num[1] {
            debug_assert!(i >= 0);
            bitset[i as usize] = true;
        }
    }
    bitset.into_iter().filter(|x| *x).count() as i32
}

// Bit Set
// 优化统计结果, 只需要遍历一次
pub fn number_of_points3(nums: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut bitset = [false; 100];
    let mut count = 0;
    for num in nums {
        #[allow(clippy::needless_range_loop)]
        for i in (num[0] as usize)..=(num[1] as usize) {
            if !bitset[i] {
                count += 1;
            }
            bitset[i] = true;
        }
    }
    count
}

// Merge Intervals
// See leetcode #0056
pub fn number_of_points4(nums: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!nums.is_empty());
    // 先对间隔的起始点排序
    let mut nums = nums;
    nums.sort_unstable_by_key(|num| num[0]);

    let mut intervals = Vec::with_capacity(nums.len());
    let mut start = nums[0][0];
    let mut end = nums[0][1];
    for num in nums.into_iter().skip(1) {
        if num[0] > end {
            // 说明有间隔, 要移动起始点和终点
            intervals.push((start, end));
            start = num[0];
            end = num[1];
        } else {
            // 没有间隔, 只移动终点
            end = end.max(num[1]);
        }
    }
    // 最后一个间隔值
    intervals.push((start, end));
    let mut count = 0;
    for (start, end) in intervals {
        count += end - start + 1;
    }
    count
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![vec![3, 6], vec![1, 5], vec![4, 7]];
    assert_eq!(func(nums), 7);

    let nums = vec![vec![1, 3], vec![5, 8]];
    assert_eq!(func(nums), 7);

    let nums = vec![vec![4, 4], vec![9, 10], vec![9, 10], vec![3, 8]];
    assert_eq!(func(nums), 8);
}

fn main() {
    check_solution(number_of_points1);
    check_solution(number_of_points2);
    check_solution(number_of_points3);
    check_solution(number_of_points4);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, number_of_points1, number_of_points2, number_of_points3, number_of_points4,
    };

    #[test]
    fn test_number_of_points1() {
        check_solution(number_of_points1);
    }

    #[test]
    fn test_number_of_points2() {
        check_solution(number_of_points2);
    }

    #[test]
    fn test_number_of_points3() {
        check_solution(number_of_points3);
    }

    #[test]
    fn test_number_of_points4() {
        check_solution(number_of_points4);
    }
}
