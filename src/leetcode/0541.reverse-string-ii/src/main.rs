// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 靠拢型双指针
pub fn reverse_str1(s: String, k: i32) -> String {
    debug_assert!(!s.is_empty());
    debug_assert!(k >= 1);

    let mut chars: Vec<char> = s.chars().collect();
    let k_usize: usize = k as usize;
    let len: usize = chars.len();
    let mut need_swap: bool = false;

    // 遍历所有字符, 以 k 个字符为间隔.
    for index in (0..len).step_by(k_usize) {
        // 判定分隔点
        if index % k_usize == 0 {
            need_swap = !need_swap;
        }

        if !need_swap {
            continue;
        }

        // 初始化双指针.
        let mut left = index;
        // 注意右侧边界.
        // 有可能 len < k, 这时就要翻转所有的字符了.
        let mut right = (index + k_usize - 1).min(len - 1);

        // 贪心, 交换所有的字符.
        while left < right {
            //println!("left: {left}, right: {right}, len: {len}");
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    chars.into_iter().collect()
}

// 使用 slice::reverse() 方法
pub fn reverse_str2(s: String, k: i32) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let k_usize: usize = k as usize;
    let len: usize = chars.len();

    // 遍历所有字符, 以 2*k 个字符为间隔.
    for index in (0..len).step_by(2 * k_usize) {
        // 注意右侧边界.
        // 有可能 len < k, 这时就要翻转所有的字符了.
        let right = (index + k_usize - 1).min(len - 1);
        chars[index..=right].reverse();
    }

    chars.into_iter().collect()
}

pub type SolutionFn = fn(String, i32) -> String;

fn check_solution(func: SolutionFn) {
    let s = "abcdefg".to_owned();
    let k = 2;
    assert_eq!(func(s, k), "bacdfeg".to_owned());

    let s = "abcd".to_owned();
    let k = 2;
    assert_eq!(func(s, k), "bacd".to_owned());

    let s = "a".to_owned();
    let k = 2;
    assert_eq!(func(s, k), "a".to_owned());

    let s = "abcdefg".to_owned();
    let k = 8;
    assert_eq!(func(s, k), "gfedcba".to_owned());
}

fn main() {
    check_solution(reverse_str1);
    check_solution(reverse_str2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, reverse_str1, reverse_str2};

    #[test]
    fn test_reverse_str1() {
        check_solution(reverse_str1);
    }

    #[test]
    fn test_reverse_str2() {
        check_solution(reverse_str2);
    }
}
