// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

// 不使用 Prefix Sum, 但是使用除法
// 时间: O(n), 空间: O(1)
pub fn product_except_self1(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut product: i32 = 1;
    let mut num_zeros: i32 = 0;
    for &num in &nums {
        if num == 0 {
            num_zeros += 1;
        } else {
            product *= num;
        }
    }

    for num in nums.iter_mut() {
        if *num == 0 {
            if num_zeros == 1 {
                *num = product;
            } else {
                *num = 0;
            }
        } else if num_zeros > 0 {
            *num = 0;
        } else {
            *num = product / *num;
        }
    }
    nums
}

// 使用模式匹配, 对上面的算法做一些优化
pub fn product_except_self2(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut product: i32 = 1;
    let mut num_zeros: usize = 0;
    for &num in &nums {
        if num == 0 {
            num_zeros += 1;
        } else {
            product *= num;
        }
    }

    for num in nums.iter_mut() {
        match (*num, num_zeros) {
            (0, 1) => *num = product,
            (0, _) => *num = 0,
            (_, num_zeros) if num_zeros > 0 => *num = 0,
            (_, _) => *num = product / *num,
        }
    }
    nums
}

// Prefix Sum
pub fn product_except_self3(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut res: Vec<i32> = vec![1; len];
    let mut product = 1;

    // 前缀的积
    for (i, num) in nums.iter().enumerate() {
        res[i] *= product;
        product *= num;
    }

    // 乘以后缀的积
    product = 1;
    for (i, num) in nums.iter().rev().enumerate() {
        res[len - i - 1] *= product;
        product *= num;
    }

    res
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 4];
    let out = func(nums);
    assert_eq!(&out, &[24, 12, 8, 6]);

    let nums = vec![-1, 1, 0, -3, 3];
    let out = func(nums);
    assert_eq!(&out, &[0, 0, 9, 0, 0]);
}

fn main() {
    check_solution(product_except_self1);
    check_solution(product_except_self2);
    check_solution(product_except_self3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, product_except_self1, product_except_self2};

    #[test]
    fn test_product_except_self1() {
        check_solution(product_except_self1);
    }

    #[test]
    fn test_product_except_self2() {
        check_solution(product_except_self2);
    }
}
