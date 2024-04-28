// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 暴力法
// 超过了内存限制, 无效
pub fn valid_palindrome1(s: String) -> bool {
    fn is_palindrome_slice(s: &[u8]) -> bool {
        s.iter().rev().copied().collect::<Vec<u8>>() == s
    }

    let mut entries = Vec::with_capacity(s.len() + 1);
    let bytes = s.as_bytes();
    entries.push(bytes.to_vec());
    for i in 0..bytes.len() {
        let mut new_bytes = Vec::new();
        new_bytes.extend_from_slice(&bytes[..i]);
        new_bytes.extend_from_slice(&bytes[i + 1..]);
        entries.push(new_bytes);
    }

    for entry in &entries {
        if is_palindrome_slice(entry) {
            return true;
        }
    }
    false
}

// 优化暴力法
// 计算超时, 无效
pub fn valid_palindrome2(s: String) -> bool {
    fn is_palindrome_slice(s: &[u8]) -> bool {
        s.iter().rev().copied().collect::<Vec<u8>>() == s
    }

    let bytes = s.as_bytes();
    if is_palindrome_slice(bytes) {
        return true;
    }

    for i in 0..bytes.len() {
        let mut new_bytes = Vec::new();
        new_bytes.extend_from_slice(&bytes[..i]);
        new_bytes.extend_from_slice(&bytes[i + 1..]);
        if is_palindrome_slice(&new_bytes) {
            return true;
        }
    }
    false
}

// 双指针法, 不需要分配新的堆内存
// 计算超时, 无效
pub fn valid_palindrome3(s: String) -> bool {
    let bytes = s.as_bytes();

    // 遍历数组, 用于跳过当前的一个元素,
    // 如果 i == bytes.len(), 则不跳过任何元素.
    for i in 0..=bytes.len() {
        // 使用靠拢型双指针来检查这个 slice 是不是回文.
        let mut left = 0;
        let mut right = bytes.len() - 1;

        let mut is_palindrome = true;
        while left < right {
            if left == i {
                left += 1;
                continue;
            }
            if right == i {
                right -= 1;
                continue;
            }

            if bytes[left] != bytes[right] {
                is_palindrome = false;
                break;
            }
            // 同时移动两个指针往中间靠拢.
            left += 1;
            right -= 1;
        }

        if is_palindrome {
            return true;
        }
    }

    false
}

// 使用两个靠拢型双指针, 不需要分配新的堆内存
pub fn valid_palindrome4(s: String) -> bool {
    fn is_palindrome(slice: &[u8], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if slice[left] != slice[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    let bytes = s.as_bytes();

    // 外层双指针, 用于遍历数组
    let mut left = 0;
    let mut right = bytes.len() - 1;
    while left < right {
        if bytes[left] != bytes[right] {
            return is_palindrome(bytes, left, right - 1) || is_palindrome(bytes, left + 1, right);
        }
        left += 1;
        right -= 1;
    }
    true
}

pub type SolutionFn = fn(String) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(&str, bool)] = &[
        ("aba", true),
        ("abca", true),
        ("abc", false),
        ("deeee", true),
    ];
    for record in RECORDS {
        assert_eq!(func(record.0.to_owned()), record.1);
    }
}

fn main() {
    check_solution(valid_palindrome1);
    check_solution(valid_palindrome2);
    check_solution(valid_palindrome3);
    check_solution(valid_palindrome4);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, valid_palindrome1, valid_palindrome2, valid_palindrome3, valid_palindrome4,
    };

    #[test]
    fn test_valid_palindrome1() {
        check_solution(valid_palindrome1);
    }

    #[test]
    fn test_valid_palindrome2() {
        check_solution(valid_palindrome2);
    }

    #[test]
    fn test_valid_palindrome3() {
        check_solution(valid_palindrome3);
    }

    #[test]
    fn test_valid_palindrome4() {
        check_solution(valid_palindrome4);
    }
}
