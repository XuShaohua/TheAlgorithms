// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute force
pub fn max_score1(s: String) -> i32 {
    debug_assert!(s.len() >= 2);

    let mut max_value = 0;
    for i in 1..s.len() {
        let count_zeros = s[0..i]
            .as_bytes()
            .iter()
            .copied()
            .filter(|byte| *byte == b'0')
            .count();
        let count_ones = s[i..]
            .as_bytes()
            .iter()
            .copied()
            .filter(|byte| *byte == b'1')
            .count();
        let sum = count_zeros + count_ones;
        max_value = max_value.max(sum);
    }
    max_value as i32
}

// Prefix sum
pub fn max_score2(s: String) -> i32 {
    debug_assert!(!s.is_empty());

    let len = s.len();

    // 从前向后统计字符 `0` 出现的次数
    let count_zeros = {
        let mut count_zeros = Vec::with_capacity(len);
        let mut last_zeros = 0;
        for &byte in s.as_bytes() {
            if byte == b'0' {
                last_zeros += 1;
            }
            count_zeros.push(last_zeros);
        }
        count_zeros
    };

    // 从后向前统计字符 `1` 出现的次数
    let count_ones = {
        let mut count_ones = Vec::with_capacity(len);
        let mut last_ones = 0;
        for &byte in s.as_bytes().iter().rev() {
            if byte == b'1' {
                last_ones += 1;
            }
            count_ones.push(last_ones);
        }
        // 将 `1` 的计数数组反转
        count_ones.reverse();
        count_ones
    };

    // 遍历计数数组, 找到最大的那个组合
    let mut max_sum = 0;
    for i in 1..len {
        // s[0..i] 计算包含 `0` 的个数
        // s[i..] 计算包含 `1` 的个数
        let sum = count_zeros[i - 1] + count_ones[i];
        max_sum = max_sum.max(sum);
    }
    max_sum
}

pub type SolutionFn = fn(String) -> i32;

fn check_solution(func: SolutionFn) {
    let s = "011101".to_owned();
    assert_eq!(func(s), 5);

    let s = "00111".to_owned();
    assert_eq!(func(s), 5);

    let s = "1111".to_owned();
    assert_eq!(func(s), 3);

    let s = "00".to_owned();
    assert_eq!(func(s), 1);

    let s = "11".to_owned();
    assert_eq!(func(s), 1);
}

fn main() {
    check_solution(max_score1);
    check_solution(max_score2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, max_score1, max_score2};

    #[test]
    fn test_max_score1() {
        check_solution(max_score1);
    }

    #[test]
    fn test_max_score2() {
        check_solution(max_score2);
    }
}
