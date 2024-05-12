// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 靠拢型双指针
pub fn reverse_words1(s: String) -> String {
    debug_assert!(!s.is_empty());

    // 先转换成字符数组.
    let mut chars: Vec<char> = s.chars().collect();

    let mut index: usize = 0;
    let mut last_word_index: usize = 0;
    let len = chars.len();

    // 遍历数组, 找到空格, 用它作分隔.
    while index < len {
        // 如果遇到空格, 或者达到了字符结尾.
        if chars[index] == ' ' || index == len - 1 {
            // 初始化双指针
            let mut left = last_word_index;
            // 如果遇到了空格, 单词的右边界是 index - 1;
            // 如果到了字符结尾, 单词的右边界就是 len - 1;
            let mut right = if index == len - 1 { index } else { index - 1 };
            last_word_index = index + 1;

            // 交换单词中的所有字符.
            while left < right {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        index += 1;
    }

    chars.into_iter().collect()
}

// slice::reverse() 来交换
// str::split_ascii_whitespace() 来分隔单词
pub fn reverse_words2(s: String) -> String {
    debug_assert!(!s.is_empty());

    let mut bytes: Vec<u8> = Vec::with_capacity(s.len());

    for word in s.split_ascii_whitespace() {
        for &byte in word.as_bytes().iter().rev() {
            bytes.push(byte);
        }
        bytes.push(b' ');
    }
    // 移除最后一个多余的空格.
    bytes.pop();
    String::from_utf8(bytes).unwrap()
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "Let's take LeetCode contest".to_owned();
    assert_eq!(func(s), "s'teL ekat edoCteeL tsetnoc".to_owned());

    let s = "Mr Ding".to_owned();
    assert_eq!(func(s), "rM gniD".to_owned());
}

fn main() {
    check_solution(reverse_words1);
    check_solution(reverse_words2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, reverse_words1, reverse_words2};

    #[test]
    fn test_reverse_words1() {
        check_solution(reverse_words1);
    }

    #[test]
    fn test_reverse_words2() {
        check_solution(reverse_words2);
    }
}
