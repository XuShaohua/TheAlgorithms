// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 双指针法
pub fn first_palindrome1(words: Vec<String>) -> String {
    fn is_palindrome(s: &str) -> bool {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len() - 1;
        while left < right {
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    for word in &words {
        if is_palindrome(word) {
            return word.clone();
        }
    }
    String::new()
}

// 反转字符串
pub fn first_palindrome2(words: Vec<String>) -> String {
    fn is_palindrome(s: &str) -> bool {
        s.chars().rev().collect::<String>() == s
    }
    for word in &words {
        if is_palindrome(word) {
            return word.clone();
        }
    }
    String::new()
}

pub type SolutionFn = fn(Vec<String>) -> String;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(&[&str], &str)] = &[
        (&["abc", "car", "ada", "racecar", "cool"], "ada"),
        (&["notapalindrome", "racecar"], "racecar"),
        (&["def", "ghi"], ""),
    ];
    for record in RECORDS {
        let vec = record.0.iter().map(|s| s.to_string()).collect();
        assert_eq!(func(vec), record.1);
    }
}

fn main() {
    check_solution(first_palindrome1);
    check_solution(first_palindrome2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, first_palindrome1, first_palindrome2};

    #[test]
    fn test_first_palindrome1() {
        check_solution(first_palindrome1);
    }

    #[test]
    fn test_first_palindrome2() {
        check_solution(first_palindrome2);
    }
}
