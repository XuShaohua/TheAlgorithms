// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 使用 str::split_ascii_whitespace().
pub fn reverse_words1(s: String) -> String {
    let parts: Vec<&str> = s.split_ascii_whitespace().rev().collect();
    parts.join(" ")
}

// 手动实现空格的分隔操作
pub fn reverse_words2(s: String) -> String {
    debug_assert!(!s.is_empty());

    let mut index: usize = 0;
    let bytes: &[u8] = s.as_bytes();
    let len: usize = bytes.len();
    let mut parts: Vec<&str> = Vec::new();

    while index < len {
        if bytes[index] != b' ' {
            // 贪心, 读取尽可能多的字符.
            let start: usize = index;
            while index < len && bytes[index] != b' ' {
                index += 1;
            }
            parts.push(&s[start..index]);
        }

        index += 1;
    }

    parts.reverse();
    parts.join(" ")
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "the sky is blue".to_owned();
    let exp = "blue is sky the".to_owned();
    assert_eq!(func(s), exp);

    let s = "  hello world  ".to_owned();
    let exp = "world hello".to_owned();
    assert_eq!(func(s), exp);

    let s = "a good   example".to_owned();
    let exp = "example good a".to_owned();
    assert_eq!(func(s), exp);
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
