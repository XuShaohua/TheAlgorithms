// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;

// Greedy
// 尽量将较大的位数往前放, 将较小的位数往后放.
pub fn maximum_swap1(num: i32) -> i32 {
    #[must_use]
    fn from_digits(digits: &[i32]) -> i32 {
        let mut num = 0;
        for &digit in digits {
            num = num * 10 + digit;
        }
        num
    }

    #[must_use]
    fn to_digits(mut num: i32) -> Vec<i32> {
        if num == 0 {
            return vec![0];
        }

        let mut digits = Vec::new();
        while num != 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        digits
    }

    debug_assert!(num >= 0);
    // 先把整数转换成各个位数.
    let mut digits: Vec<i32> = to_digits(num);
    let mut left: usize = 0;

    // 对整数位按照从大到小进行排序, 并记录其索引位置.
    let mut sorted_digits: Vec<(usize, i32)> = digits.iter().copied().enumerate().collect();
    sorted_digits.sort_unstable_by(|a: &(usize, i32), b: &(usize, i32)| match b.1.cmp(&a.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
    });
    //println!("digits: {digits:?}, sorted: {sorted_digits:?}");

    // 再从中找出最大的位数, 如果不是处于第一位, 就与第二位交换.
    // 如果最大的位数是第一位, 就找出第二大的位数, 如果它不处于第二位, 就与第二位交换.
    // 依次做下去, 直到遍历所有整数位.
    while left < digits.len() {
        if digits[left] != sorted_digits[left].1 {
            // 考虑重复的位数, 如果有重复的位数, 就选索引值最高的那个.
            let mut right = sorted_digits[left].0;
            let mut i = left;
            while i + 1 < digits.len() && sorted_digits[i].1 == sorted_digits[i + 1].1 {
                right = sorted_digits[i + 1].0;
                i += 1;
            }
            digits.swap(left, right);
            break;
        }
        left += 1;
    }

    from_digits(&digits)
}

pub type SolutionFn = fn(i32) -> i32;

fn check_maximum_swap(func: SolutionFn) {
    assert_eq!(func(2736), 7236);
    assert_eq!(func(9973), 9973);
    assert_eq!(func(98368), 98863);
    assert_eq!(func(1993), 9913);
}

fn main() {
    check_maximum_swap(maximum_swap1);
}

#[cfg(test)]
mod tests {
    use super::{check_maximum_swap, maximum_swap1};

    #[test]
    fn test_maximum_swap1() {
        check_maximum_swap(maximum_swap1);
    }
}
