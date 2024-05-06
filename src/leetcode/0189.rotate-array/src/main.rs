// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

use std::mem;

// Brute force
#[allow(clippy::needless_range_loop)]
pub fn rotate1(nums: &mut Vec<i32>, k: i32) {
    // 检查边界条件
    if nums.is_empty() || k <= 0 {
        return;
    }
    let len: usize = nums.len();
    let k: usize = (k as usize) % len;
    if k == 0 {
        return;
    }

    for _ in 0..k {
        let mut temp: i32 = nums[len - 1];
        for i in 0..(len - 1) {
            mem::swap(&mut temp, &mut nums[i]);
        }
        nums[len - 1] = temp;
    }
}

// 三次翻转.
pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
    // 检查边界条件
    if nums.is_empty() || k <= 0 {
        return;
    }
    let len: usize = nums.len();
    let k: usize = (k as usize) % len;
    if k == 0 {
        return;
    }

    // 第一步, 把所有元素做翻转.
    nums.reverse();

    // 第二步, 找到右移的分界线 k, 把 [0..k] 做翻转; 把 [k..len] 做翻转
    nums[0..k].reverse();
    nums[k..].reverse();
}

// 三次翻转, 但使用靠拢型双指针实现反转函数.
pub fn rotate3(nums: &mut Vec<i32>, k: i32) {
    fn reverse_array(nums: &mut [i32], mut start: usize, mut end: usize) {
        let mut temp;
        while start < end {
            // nums.swap(start, end);
            temp = nums[start];
            nums[start] = nums[end];
            nums[end] = temp;
            start += 1;
            end -= 1;
        }
    }

    // 检查边界条件
    if nums.is_empty() || k <= 0 {
        return;
    }
    let len: usize = nums.len();
    let k: usize = (k as usize) % len;
    if k == 0 {
        return;
    }

    // 第一步, 把所有元素做翻转.
    reverse_array(nums, 0, len - 1);

    // 第二步, 找到右移的分界线 k, 把 [0..k] 做翻转; 把 [k..len] 做翻转
    reverse_array(nums, 0, k - 1);
    reverse_array(nums, k, len - 1);
}

pub type SolutionFn = fn(&mut Vec<i32>, i32);

fn check_solution(func: SolutionFn) {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    func(&mut nums, k);
    assert_eq!(&nums, &[5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![-1, -100, 3, 99];
    let k = 2;
    func(&mut nums, k);
    assert_eq!(&nums, &[3, 99, -1, -100]);
}

fn main() {
    check_solution(rotate1);
    check_solution(rotate2);
    check_solution(rotate3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, rotate1};

    #[test]
    fn test_rotate1() {
        check_solution(rotate1);
    }
}
