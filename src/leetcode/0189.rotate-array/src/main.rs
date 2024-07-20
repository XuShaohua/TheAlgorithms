// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

use std::mem;

/// Brute force
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

/// 三次反转法
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

    // 第一步, 把所有元素做反转.
    nums.reverse();
    // 第二步, 找到右移的分界线 k, 把 [0..k] 做反转.
    nums[0..k].reverse();
    // 第三步, 把 [k..len] 做反转
    nums[k..].reverse();
}

/// 使用临时数组
pub fn rotate3(nums: &mut Vec<i32>, k: i32) {
    if nums.is_empty() || k <= 0 {
        return;
    }
    let len = nums.len();
    let k = len - (k as usize) % len;
    if k == 0 {
        return;
    }

    let mut aux = Vec::with_capacity(len);
    aux.extend_from_slice(&nums[k..]);
    aux.extend_from_slice(&nums[..k]);
    mem::swap(nums, &mut aux);
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
    use super::{check_solution, rotate1, rotate2, rotate3};

    #[test]
    fn test_rotate1() {
        check_solution(rotate1);
    }

    #[test]
    fn test_rotate2() {
        check_solution(rotate2);
    }

    #[test]
    fn test_rotate3() {
        check_solution(rotate3);
    }
}
