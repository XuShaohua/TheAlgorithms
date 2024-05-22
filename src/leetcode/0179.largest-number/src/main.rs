// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;

// Sorting
// 自定义排序函数
pub fn largest_number1(nums: Vec<i32>) -> String {
    #[allow(clippy::ptr_arg)]
    fn sort_digits(a: &String, b: &String) -> Ordering {
        let len = a.len() + b.len();
        let mut ab = String::with_capacity(len);
        ab.push_str(a);
        ab.push_str(b);

        let mut ba = String::with_capacity(len);
        ba.push_str(b);
        ba.push_str(a);

        ba.cmp(&ab)
    }

    // 转换成字符串
    let mut nums_str: Vec<String> = nums.iter().map(i32::to_string).collect();
    // 调用自定义排序函数
    nums_str.sort_by(sort_digits);

    let mut out = String::new();
    for s in &nums_str {
        out.push_str(s);
    }

    // 处理特殊情况, 全部都是 0.
    if out.starts_with('0') {
        "0".to_owned()
    } else {
        out
    }
}

// Sorting
// 自定义排序函数
// 这次不使用字符串, 而是使用 digits
pub fn largest_number2(nums: Vec<i32>) -> String {
    fn to_digits(mut num: i32) -> Vec<i32> {
        debug_assert!(num >= 0);
        // 处理特殊情况
        if num == 0 {
            return vec![0];
        }

        let mut digits = Vec::with_capacity(11);
        while num != 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();

        digits
    }

    fn sort_digits(a: &i32, b: &i32) -> Ordering {
        let a = *a;
        let b = *b;
        if a == b {
            return Ordering::Equal;
        }

        // 先转换成 digits
        let a_digits = to_digits(a);
        let b_digits = to_digits(b);

        // 然后组合 ab, ba 两个值. 转换成 i64, 避免溢出.
        let ab: i64 = b_digits
            .iter()
            .fold(a as i64, |acc, digit| acc * 10 + *digit as i64);
        let ba: i64 = a_digits
            .iter()
            .fold(b as i64, |acc, digit| acc * 10 + *digit as i64);

        // 最后比较它们的大小
        ba.cmp(&ab)
    }

    // 先处理特殊情况, 全都是 0
    if nums.iter().all(|num| *num == 0) {
        return "0".to_owned();
    }

    let mut nums = nums;
    nums.sort_by(sort_digits);

    let mut out = String::new();
    for num in nums {
        out.push_str(&num.to_string());
    }

    out
}

pub type SolutionFn = fn(Vec<i32>) -> String;

fn check_solution(func: SolutionFn) {
    let nums = vec![10, 2];
    assert_eq!(func(nums), "210");

    let nums = vec![3, 30, 34, 5, 9];
    assert_eq!(func(nums), "9534330");

    let nums = vec![0, 0];
    assert_eq!(func(nums), "0");

    let nums = vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    assert_eq!(func(nums), "9876543210");
}

fn main() {
    check_solution(largest_number1);
    check_solution(largest_number2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, largest_number1, largest_number2};

    #[test]
    fn test_largest_number1() {
        check_solution(largest_number1);
    }

    #[test]
    fn test_largest_number2() {
        check_solution(largest_number2);
    }
}
